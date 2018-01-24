mod coordinator;

#[cfg(test)]
mod tests {
    use coordinator::process;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        process();
    }
}
