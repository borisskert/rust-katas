#[cfg(test)]
mod sample_tests {
    use rust_katas::pagination_helper::PaginationHelper;

    #[test]
    fn test_item_count() {
        let helper = PaginationHelper::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3);
        assert_eq!(helper.item_count(), 11);
    }

    #[test]
    fn test_page_count() {
        let helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(helper.page_count(), 2);
    }

    #[test]
    fn test_page_item_count() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_item_count(1), Some(10));
        assert_eq!(helper.page_item_count(2), Some(4));
    }

    #[test]
    fn test_page_index() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_index(40), None);
        assert_eq!(helper.page_index(22), Some(2));
    }
}
