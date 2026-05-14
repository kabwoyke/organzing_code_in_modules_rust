mod DB {

    mod read {
        pub fn fetch_all(){}
        fn fetch_by_id(id:i32){}

    }

    

    mod write {
        fn insert(data:String){
            super::read::fetch_all();
        }
        fn update(id:i32){}
        fn delete(){}

    }
}
