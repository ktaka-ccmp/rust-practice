mod pizza_order{
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: "regular dough".to_string(),
                cheese: "mozzarella".to_string(),
                topping: topping.to_string(),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Please have a seat at the table.");
        }
        pub fn take_order() {
            seat_at_table();
            let cust_pizza = super::Pizza::lunch("pepperoni");
            serve_customer(cust_pizza);
            fn serve_customer(cust_pizza: super::Pizza) {
                println!("Order is ready with {} topping", cust_pizza.topping);
            }

        }

    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
