struct BrowserHistory {
    history: Vec<String>,
    current: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        let mut result = BrowserHistory {
            history: vec![],
            current: 0,
        };

        result.visit(homepage);
        result
    }

    fn visit(&mut self, url: String) {
        while self.current < self.history.len() {
            self.history.pop();
        }

        self.history.push(url);
        self.current += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        let result = self.history[self.current - steps as usize - 1].clone();
        self.current -= steps as usize ;

        return result;
    }

    fn forward(&mut self, steps: i32) -> String {
        let result = self.history[self.current + steps as usize - 1].clone();
        self.current += steps as usize ;

        return result;
    }
}

#[test]
fn test_code_1472() {
    let mut browser_history = BrowserHistory::new("main".to_owned());
    browser_history.visit("www.baidu1.com".into());
    browser_history.visit("www.baidu2.com".into());
    browser_history.visit("www.baidu3.com".into());

    assert_eq!(browser_history.back(2), "www.baidu1.com".to_owned());

    browser_history.visit("www.baidu3.com".into());
    browser_history.visit("www.baidu2.com".into());
    browser_history.visit("www.baidu1.com".into());

    assert_eq!(browser_history.back(4), "main".to_owned());
}
