import java.lang.System;
import java.lang.Deprecated;
import java.lang.RuntimeException;

public final class Hello {
   @Deprecated
   public static final String MESSAGE = "Hello World!";
   public static void main(String[] args) {
      System.out.println(MESSAGE);
   }

   public static void stackmapper(int value) {
        int i = 0;
        int j = 0;
        if (i > 0) {
            long k = 0;
            if (j == 0) {
                k++;
                int s=1111;
            }
            int t = 0;
        }
   }

   @HelloAnno(value = "Meow")
   public void thrower() throws RuntimeException {}

   public interface HelloInterface {
      void print(@HelloAnno(value = "Meow") String value);
   }

   public @interface HelloAnno {
      String value();
   }

   @Deprecated
   public class InnerHello implements HelloInterface {
      public int intMethod() { return -1; }
      public void intMethod(int value) {}
      public String stringMethod() { return "meow"; }
      public void stringMethod(String value) {}

      @Override
      public void print(String value) {
         System.out.println(value);
      }
   }
}
