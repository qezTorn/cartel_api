#[derive(Debug)]
pub struct Range{
    from: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug)]
pub struct RangeBuilder{
    from: Option<i64>,
    to: Option<i64>,
}

impl RangeBuilder{
    pub fn new() -> RangeBuilder{
        RangeBuilder{
            from: None,
            to: None,
        }
    }

    pub fn from(&mut self, from: i64) -> &mut Self{
        self.from = Some(from);
        self
    }

    pub fn to(&mut self, to: i64) -> &mut Self{
        self.to = Some(to);
        self
    }

    pub fn build(&mut self) -> Range{
        Range{
            from: self.from,
            to: self.to,
        }
    }
}

impl Range{
    pub fn new() -> RangeBuilder{
        RangeBuilder{
            from: None,
            to: None,
        }
    }

    //temp eventually replace with proper into<string> or .toString implementation
    pub fn use_range(&self) -> String{
        let mut output = "".to_string();

        if self.from.is_some(){
            output = format!("{}&from={}", output, self.from.unwrap());
        }

        if self.to.is_some(){
            output = format!("{}&to={}", output, self.to.unwrap());
        }

        output.to_string()
    }
}
