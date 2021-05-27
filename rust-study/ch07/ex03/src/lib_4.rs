fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // back of house 의 모듈인 cook order 은 pub 가 아니라도 참조 가능
        cook_order();
        // super 를 사용해 부모 모듈에서 시작하는 상대 경로를 구성이 가능하다.
        super::serve_order();
    }

    fn cook_order() {}
}
