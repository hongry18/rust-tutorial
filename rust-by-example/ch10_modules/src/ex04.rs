/// default function
fn function() {
    println!("called `function()`");
}

/// cool module
mod cool {
    /// cool::function()
    pub fn function() {
        println!("called `cool::function()`");
    }
}

/// my module
mod my {
    /// my::function()
    fn function() {
        println!("called `my::function()`");
    }

    /// my::cool
    mod cool {
        /// my::cool::function
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    // my::indirect_call()
    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();

        self::cool::function();
        super::function();

        {
            use crate::ex04::cool::function as root_function;
            root_function();
        }
    }
}

/// ex04 run
pub fn run() {
    my::indirect_call();
}
