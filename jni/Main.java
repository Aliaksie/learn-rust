public class Main {
    private static native String hello(String input);

    static {
        System.loadLibrary("jni");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        String output = Main.hello("RUST!");
        System.out.println(output);
    }
}

