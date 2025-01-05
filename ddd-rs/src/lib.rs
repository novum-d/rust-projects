mod value_object;

#[cfg(test)]
mod tests {

    mod value_object {
        use crate::value_object::FullName;

        /**
         * value-object: システム固有の値を表現するために定義されたオブジェクト
         **/

        /* 名前を表示する例。*/
        #[test]
        fn bad() {
            // 世界には姓が先に名が後にくる氏名が存在するため、失敗する
            let full_name = "hamada tomoki";
            let tokens = full_name.split(" ").collect::<Vec<&str>>();
            let first_name = tokens[0];
            assert_eq!(first_name, "tomoki");
        }

        #[test]
        fn good() {
            let full_name = FullName::new("tomoki", "hamada");
            assert_eq!(full_name.first_name(), "tomoki");
        }
    }
}
