use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn contract_state_for_testing() {
    test_transform!(test_hover,r#"
    use Balance::contr<caret>act_state_for_testing;

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref self: ContractState, value_: u128) {
            self.value.write(value_);
        }
    }
    "#,@r#"
    source_context = """
    use Balance::contr<caret>act_state_for_testing;
    """
    highlight = """
    use Balance::<sel>contract_state_for_testing</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    pub fn contract_state_for_testing() -> ContractState
    ```
    """
    "#)
}

#[test]
fn constructor_ref_self() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref se<caret>lf: ContractState, value_: u128) {
            self.value.write(value_);
        }
    }
    "#,@r#"
    source_context = """
        fn constructor(ref se<caret>lf: ContractState, value_: u128) {
    """
    highlight = """
        fn constructor(ref <sel>self</sel>: ContractState, value_: u128) {
    """
    popover = """
    ```cairo
    ref self: hello::Balance::ContractState
    ```
    """
    "#)
}

#[test]
fn constructor_self_type() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref self: Contra<caret>ctState, value_: u128) {
            self.value.write(value_);
        }
    }
    "#,@r#"
    source_context = """
        fn constructor(ref self: Contra<caret>ctState, value_: u128) {
    """
    highlight = """
        fn constructor(ref self: <sel>ContractState</sel>, value_: u128) {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    pub struct ContractState {}
    ```
    """
    "#)
}

#[test]
fn constructor_storage_value_access() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref self: ContractState, value_: u128) {
            self.val<caret>ue.write(value_);
        }
    }
    "#,@r#"
    source_context = """
            self.val<caret>ue.write(value_);
    """
    highlight = """
            self.<sel>value</sel>.write(value_);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    #[derive(Drop, Copy)]
    #[doc(hidden)]
    pub struct StorageStorageBaseMut {
        pub value: starknet::storage::StorageBase<starknet::storage::Mutable<u128>>,
    }
    ```
    """
    "#)
}

#[test]
fn constructor_storage_value_write() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref self: ContractState, value_: u128) {
            self.value.write(<caret>value_);
        }
    }
    "#,@r#"
    source_context = """
            self.value.write(<caret>value_);
    """
    highlight = """
            self.value.write(<sel>value_</sel>);
    """
    popover = """
    ```cairo
    value_: core::integer::u128
    ```
    """
    "#)
}

#[test]
fn interface_usage() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract interface.
    #[starknet::interface]
    trait IBalance<T> {
        /// Returns the current balance.
        fn get(self: @T) -> u128;
        /// Increases the balance by the given amount.
        fn increase(ref self: T, a: u128);
    }

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref self: ContractState, value_: u128) { }

        #[abi(embed_v0)]
        impl Balance of super::IBa<caret>lance<ContractState> {
            fn get(self: @ContractState) -> u128 {
                self.value.read()
            }
            fn increase(ref self: ContractState, a: u128)  {
                self.value.write(self.value.read() + a );
            }
        }
    }
    "#,@r#"
    source_context = """
        impl Balance of super::IBa<caret>lance<ContractState> {
    """
    highlight = """
        impl Balance of super::<sel>IBalance</sel><ContractState> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait IBalance<T>
    ```
    ---
    The balance contract interface."""
    "#)
}

#[test]
fn interface_usage_generic_arg() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract interface.
    #[starknet::interface]
    trait IBalance<T> {
        /// Returns the current balance.
        fn get(self: @T) -> u128;
        /// Increases the balance by the given amount.
        fn increase(ref self: T, a: u128);
    }

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[constructor]
        fn constructor(ref self: ContractState, value_: u128) { }

        #[abi(embed_v0)]
        impl Balance of super::IBalance<Con<caret>tractState> {
            fn get(self: @ContractState) -> u128 {
                self.value.read()
            }
            fn increase(ref self: ContractState, a: u128)  {
                self.value.write(self.value.read() + a );
            }
        }
    }
    "#,@r#"
    source_context = """
        impl Balance of super::IBalance<Con<caret>tractState> {
    """
    highlight = """
        impl Balance of super::IBalance<<sel>ContractState</sel>> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    pub struct ContractState {}
    ```
    """
    "#)
}

#[test]
fn read_in_interafce_impl() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract interface.
    #[starknet::interface]
    trait IBalance<T> {
        /// Returns the current balance.
        fn get(self: @T) -> u128;
        /// Increases the balance by the given amount.
        fn increase(ref self: T, a: u128);
    }

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[abi(embed_v0)]
        impl Balance of super::IBalance<ContractState> {
            fn get(self: @ContractState) -> u128 {
                self.value.r<caret>ead()
            }
            fn increase(ref self: ContractState, a: u128)  {
                self.value.write( self.value.read() + a );
            }
        }
    }
    "#,@r#"
    source_context = """
                self.value.r<caret>ead()
    """
    highlight = """
                self.value.<sel>read</sel>()
    """
    popover = """
    ```cairo
    core::starknet::storage::StoragePointerReadAccess
    ```
    ```cairo
    pub trait StoragePointerReadAccess<T>
    fn read(self: @T) -> Self::Value
    ```
    """
    "#)
}

#[test]
fn write_in_interafce_impl() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract interface.
    #[starknet::interface]
    trait IBalance<T> {
        /// Returns the current balance.
        fn get(self: @T) -> u128;
        /// Increases the balance by the given amount.
        fn increase(ref self: T, a: u128);
    }

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[abi(embed_v0)]
        impl Balance of super::IBalance<ContractState> {
            fn get(self: @ContractState) -> u128 {
                self.value.read()
            }
            fn increase(ref self: ContractState, a: u128)  {
                self.value.wr<caret>ite( self.value.read() + a );
            }
        }
    }
    "#,@r#"
    source_context = """
                self.value.wr<caret>ite( self.value.read() + a );
    """
    highlight = """
                self.value.<sel>write</sel>( self.value.read() + a );
    """
    popover = """
    ```cairo
    core::starknet::storage::StoragePointerWriteAccess
    ```
    ```cairo
    pub trait StoragePointerWriteAccess<T>
    fn write(self: T, value: Self::Value)
    ```
    """
    "#)
}

#[test]
fn value_in_interafce_impl() {
    test_transform!(test_hover,r#"
    use Balance::contract_state_for_testing;

    /// The balance contract interface.
    #[starknet::interface]
    trait IBalance<T> {
        /// Returns the current balance.
        fn get(self: @T) -> u128;
        /// Increases the balance by the given amount.
        fn increase(ref self: T, a: u128);
    }

    /// The balance contract.
    #[starknet::contract]
    mod Balance {
        use core::traits::Into;

        #[storage]
        struct Storage {
            /// Storage value.
            value: u128,
        }

        #[abi(embed_v0)]
        impl Balance of super::IBalance<ContractState> {
            fn get(self: @ContractState) -> u128 {
                self.value.read()
            }
            fn increase(ref self: ContractState, a: u128)  {
                self.value.write( self.value.read() + <caret>a );
            }
        }
    }
    "#,@r#"
    source_context = """
                self.value.write( self.value.read() + <caret>a );
    """
    highlight = """
                self.value.write( self.value.read() + <sel>a</sel> );
    """
    popover = """
    ```cairo
    a: core::integer::u128
    ```
    """
    "#)
}
