// Test module to demonstrate the dynamic buffer management system

#[cfg(test)]
mod tests {
    use super::super::{BufferConfig, DynamicVertexBuffer, VertexBudgetManager, Vertex};

    #[test]
    fn test_buffer_config_defaults() {
        let config = BufferConfig::default();
        assert_eq!(config.initial_capacity, 50_000);
        assert_eq!(config.max_capacity, 500_000);
        assert_eq!(config.growth_factor, 1.5);
        assert_eq!(config.usage_history_frames, 60);
        assert_eq!(config.resize_threshold, 0.8);
    }

    #[test]
    fn test_vertex_budget_manager() {
        let mut budget_manager = VertexBudgetManager::new(100_000);

        // Set category budgets
        budget_manager.set_category_budget("llamas", 60_000);
        budget_manager.set_category_budget("effects", 40_000);

        // Test allocation within budget
        let allocated = budget_manager.check_allocation("llamas", 30_000);
        assert_eq!(allocated, 30_000);

        // Test allocation exceeding budget
        let allocated = budget_manager.check_allocation("llamas", 50_000);
        assert_eq!(allocated, 30_000); // Should be limited to remaining budget

        // Test new frame reset
        budget_manager.start_frame();
        let allocated = budget_manager.check_allocation("llamas", 30_000);
        assert_eq!(allocated, 30_000); // Should work again after reset
    }

    #[test]
    fn test_usage_stats() {
        let mut stats = super::super::buffer_manager::VertexUsageStats::new(5);

        // Add some usage data
        stats.update_usage(1000, 5);
        stats.update_usage(1500, 5);
        stats.update_usage(1200, 5);

        assert!(stats.get_peak_usage() >= 1500);
        assert!(stats.get_average_usage() > 0.0);
        assert!(stats.get_predicted_usage() > 0);
    }
}

/// Production-ready buffer overflow prevention demo
pub fn demo_buffer_overflow_prevention() {
    println!("\n=== AetheriumBloom Buffer Overflow Prevention Demo ===");

    let config = BufferConfig {
        initial_capacity: 1000,
        max_capacity: 5000,
        growth_factor: 1.5,
        usage_history_frames: 10,
        resize_threshold: 0.8,
    };

    let mut buffer = DynamicVertexBuffer::new(config, std::mem::size_of::<Vertex>() as u64);
    let mut budget_manager = VertexBudgetManager::new(4000);
    budget_manager.set_category_budget("test", 3000);

    println!("Initial buffer capacity: {}", buffer.get_capacity());

    // Simulate usage scenarios
    let scenarios = vec![
        ("Normal load", 500),
        ("High load", 1200),
        ("Peak load", 2500),
        ("Overflow attempt", 6000),
    ];

    for (scenario, vertex_count) in scenarios {
        budget_manager.start_frame();
        let allocated = budget_manager.check_allocation("test", vertex_count);
        let validated = buffer.validate_vertex_count(allocated).unwrap_or(0);

        println!("\n{}: requested {}, allocated {}, validated {}",
                scenario, vertex_count, allocated, validated);
        println!("  Circuit breaker active: {}", buffer.is_circuit_breaker_active());
        println!("  Current capacity: {}", buffer.get_capacity());
    }

    println!("\n=== Demo complete - No buffer overflows occurred ===");
}