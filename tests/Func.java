class Main {
    public static void main(String[] args) {
        int i = fact(6);
        System.out.println(i);
    }
    static int test(int d) {
        return d + 2;
    }

    static int fact(int a) {
        if (a <= 1) {
            return 1;
        }
        return a * fact(a - 1);
    }
}

