mod DB {

    pub mod read {
        pub fn fetch_all(){}
        fn fetch_by_id(id:i32){}

    }

    
   

    mod write {
        use crate::DB::read::fetch_all;
        fn insert(data:String){
            fetch_all();
        }
        fn update(id:i32){}
        fn delete(){}

    }
}
