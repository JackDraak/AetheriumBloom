// Dynamic Buffer Management System for AetheriumBloom
// Implements architect's recommendations for production-ready resource management

use anyhow::Result;
use wgpu::*;
use std::collections::VecDeque;

/// Configuration for buffer management behavior
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Initial buffer size in vertices
    pub initial_capacity: usize,
    /// Maximum buffer size before circuit breaker activates
    pub max_capacity: usize,
    /// Growth factor when resizing (e.g., 1.5 = 50% increase)
    pub growth_factor: f32,
    /// Number of frames to track for usage prediction
    pub usage_history_frames: usize,
    /// Threshold for triggering resize (e.g., 0.8 = resize when 80% full)
    pub resize_threshold: f32,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            initial_capacity: 50_000,
            max_capacity: 500_000,
            growth_factor: 1.5,
            usage_history_frames: 60, // 1 second at 60 FPS
            resize_threshold: 0.8,
        }
    }
}

/// Tracks vertex usage statistics for predictive allocation
#[derive(Debug, Clone)]
pub struct VertexUsageStats {
    /// Historical vertex counts over recent frames
    usage_history: VecDeque<usize>,
    /// Current peak usage within history window
    peak_usage: usize,
    /// Average usage over history window
    average_usage: f32,
    /// Predicted next frame usage based on trend
    predicted_usage: usize,
}

impl VertexUsageStats {
    pub fn new(history_frames: usize) -> Self {
        Self {
            usage_history: VecDeque::with_capacity(history_frames),
            peak_usage: 0,
            average_usage: 0.0,
            predicted_usage: 0,
        }
    }

    /// Update statistics with current frame's vertex usage
    pub fn update_usage(&mut self, vertex_count: usize, max_history: usize) {
        // Add current usage to history
        self.usage_history.push_back(vertex_count);

        // Maintain history window size
        if self.usage_history.len() > max_history {
            self.usage_history.pop_front();
        }

        // Recalculate statistics
        self.recalculate_stats();
    }

    fn recalculate_stats(&mut self) {
        if self.usage_history.is_empty() {
            return;
        }

        // Calculate peak and average
        self.peak_usage = self.usage_history.iter().copied().max().unwrap_or(0);
        self.average_usage = self.usage_history.iter().sum::<usize>() as f32 / self.usage_history.len() as f32;

        // Simple trend-based prediction: use weighted average with recent values having more weight
        if self.usage_history.len() >= 3 {
            let recent_trend = self.calculate_trend();
            self.predicted_usage = (self.average_usage + recent_trend).max(0.0) as usize;
        } else {
            self.predicted_usage = self.average_usage as usize;
        }
    }

    fn calculate_trend(&self) -> f32 {
        if self.usage_history.len() < 3 {
            return 0.0;
        }

        let recent: Vec<_> = self.usage_history.iter().rev().take(3).collect();
        let trend = (*recent[0] as f32 - *recent[2] as f32) / 2.0;
        trend
    }

    pub fn get_peak_usage(&self) -> usize {
        self.peak_usage
    }

    pub fn get_predicted_usage(&self) -> usize {
        self.predicted_usage
    }

    pub fn get_average_usage(&self) -> f32 {
        self.average_usage
    }
}

/// Dynamic vertex buffer that can grow based on usage patterns
pub struct DynamicVertexBuffer {
    buffer: Option<Buffer>,
    current_capacity: usize,
    config: BufferConfig,
    usage_stats: VertexUsageStats,
    vertex_stride: u64,
    /// Circuit breaker: prevents allocation beyond safe limits
    circuit_breaker_active: bool,
}

impl DynamicVertexBuffer {
    pub fn new(config: BufferConfig, vertex_stride: u64) -> Self {
        Self {
            buffer: None,
            current_capacity: config.initial_capacity,
            usage_stats: VertexUsageStats::new(config.usage_history_frames),
            config,
            vertex_stride,
            circuit_breaker_active: false,
        }
    }

    /// Initialize or resize the buffer if needed
    pub fn ensure_capacity(&mut self, device: &Device, required_vertices: usize) -> Result<()> {
        // Update usage statistics
        self.usage_stats.update_usage(required_vertices, self.config.usage_history_frames);

        // Check if we need to resize
        let needs_resize = self.buffer.is_none() ||
                          required_vertices > self.current_capacity ||
                          self.should_preemptive_resize(required_vertices);

        if needs_resize {
            self.resize_buffer(device, required_vertices)?;
        }

        Ok(())
    }

    fn should_preemptive_resize(&self, current_usage: usize) -> bool {
        // Preemptive resize based on predicted usage and threshold
        let usage_ratio = current_usage as f32 / self.current_capacity as f32;
        let predicted_usage = self.usage_stats.get_predicted_usage();
        let predicted_ratio = predicted_usage as f32 / self.current_capacity as f32;

        usage_ratio >= self.config.resize_threshold || predicted_ratio >= self.config.resize_threshold
    }

    fn resize_buffer(&mut self, device: &Device, required_vertices: usize) -> Result<()> {
        // Calculate new capacity
        let predicted_need = self.usage_stats.get_predicted_usage().max(required_vertices);
        let new_capacity = self.calculate_new_capacity(predicted_need);

        // Circuit breaker check
        if new_capacity > self.config.max_capacity {
            println!("WARNING: Buffer resize blocked by circuit breaker: requested {} > max {}",
                      new_capacity, self.config.max_capacity);
            self.circuit_breaker_active = true;
            return Ok(()); // Don't fail, just use existing buffer
        }

        // Create new buffer
        let buffer_size = new_capacity as u64 * self.vertex_stride;
        let new_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Dynamic Vertex Buffer"),
            size: buffer_size,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        println!("Resized vertex buffer: {} -> {} vertices ({} bytes)",
                  self.current_capacity, new_capacity, buffer_size);

        self.buffer = Some(new_buffer);
        self.current_capacity = new_capacity;
        self.circuit_breaker_active = false;

        Ok(())
    }

    fn calculate_new_capacity(&self, required: usize) -> usize {
        // Use growth factor, but ensure we meet minimum requirement
        let growth_based = (self.current_capacity as f32 * self.config.growth_factor) as usize;
        let capacity = growth_based.max(required);

        // Add some headroom based on usage volatility
        let peak_usage = self.usage_stats.get_peak_usage();
        let headroom = (peak_usage as f32 * 0.2) as usize; // 20% headroom

        (capacity + headroom).min(self.config.max_capacity)
    }

    /// Get the current buffer, if available
    pub fn get_buffer(&self) -> Option<&Buffer> {
        self.buffer.as_ref()
    }

    /// Get current capacity in vertices
    pub fn get_capacity(&self) -> usize {
        self.current_capacity
    }

    /// Check if circuit breaker is active
    pub fn is_circuit_breaker_active(&self) -> bool {
        self.circuit_breaker_active
    }

    /// Get usage statistics for monitoring
    pub fn get_usage_stats(&self) -> &VertexUsageStats {
        &self.usage_stats
    }

    /// Validate vertex count against current capacity
    pub fn validate_vertex_count(&self, vertex_count: usize) -> Result<usize> {
        if self.circuit_breaker_active && vertex_count > self.current_capacity {
            println!("Vertex count {} exceeds capacity {} with circuit breaker active, truncating",
                      vertex_count, self.current_capacity);
            Ok(self.current_capacity)
        } else if vertex_count > self.current_capacity {
            println!("Vertex count {} exceeds capacity {}, truncating",
                      vertex_count, self.current_capacity);
            Ok(self.current_capacity)
        } else {
            Ok(vertex_count)
        }
    }
}

/// Vertex Budget Manager - tracks and limits vertex usage across render categories
#[derive(Debug)]
pub struct VertexBudgetManager {
    /// Total vertex budget per frame
    total_budget: usize,
    /// Budget allocation per category
    category_budgets: std::collections::HashMap<String, usize>,
    /// Current usage per category
    category_usage: std::collections::HashMap<String, usize>,
    /// Frame usage history for adaptive budgeting
    frame_history: VecDeque<usize>,
}

impl VertexBudgetManager {
    pub fn new(total_budget: usize) -> Self {
        Self {
            total_budget,
            category_budgets: std::collections::HashMap::new(),
            category_usage: std::collections::HashMap::new(),
            frame_history: VecDeque::with_capacity(60),
        }
    }

    /// Set budget for a specific rendering category
    pub fn set_category_budget(&mut self, category: &str, budget: usize) {
        self.category_budgets.insert(category.to_string(), budget);
    }

    /// Check if vertex allocation is within budget for category
    pub fn check_allocation(&mut self, category: &str, vertex_count: usize) -> usize {
        let budget = self.category_budgets.get(category).copied().unwrap_or(self.total_budget / 4);
        let current_usage = self.category_usage.get(category).copied().unwrap_or(0);

        let available = budget.saturating_sub(current_usage);
        let allocated = vertex_count.min(available);

        if allocated < vertex_count {
            println!("Vertex allocation limited for {}: requested {}, allocated {}",
                       category, vertex_count, allocated);
        }

        self.category_usage.insert(category.to_string(), current_usage + allocated);
        allocated
    }

    /// Reset usage counters for new frame
    pub fn start_frame(&mut self) {
        let total_usage: usize = self.category_usage.values().sum();
        self.frame_history.push_back(total_usage);

        if self.frame_history.len() > 60 {
            self.frame_history.pop_front();
        }

        self.category_usage.clear();
    }

    /// Get average vertex usage over recent frames
    pub fn get_average_usage(&self) -> f32 {
        if self.frame_history.is_empty() {
            0.0
        } else {
            self.frame_history.iter().sum::<usize>() as f32 / self.frame_history.len() as f32
        }
    }

    /// Get total budget
    pub fn get_total_budget(&self) -> usize {
        self.total_budget
    }
}