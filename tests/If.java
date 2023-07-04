class Main {
    public static void main(String[] args) {
        if (test(-1) <= 2) {
            System.out.println("hello");
        } else {
            System.out.println("world");
        }

        if (test(0) < 2) {
            System.out.println("hello2");
        } else {
            System.out.println("world2");
        }
    }

    static int test(int d) {
        return d + 2;
    }
}

