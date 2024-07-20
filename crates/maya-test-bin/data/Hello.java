import java.lang.System;
import java.lang.Deprecated;
import java.lang.RuntimeException;
import java.util.function.Supplier;

public final class Hello {
   @Deprecated
   public static final String MESSAGE = "Hello World!";
   public static void main(String[] args) {
      System.out.println(MESSAGE);
   }

   @HelloAnnoRec(value = @HelloAnno(value = "Hi"))
   public static <T> void stackmapper(int value, T ty) {
      int i = 0;
      int j = 0;
      T ty2 = ty;
      if (i > 0) {
         long k = 0;
         if (j == 0) {
            k++;
            int s=1111;
         }
         String t = "fucker";
      }
      Supplier<String> sup = () -> {
         return "fuck";
      };
      HelloInterface h = new HelloInterface() {
         @Override public void print(String value) {}
      };
   }

   @HelloAnno(value = "Meow")
   public void thrower() throws RuntimeException {}

   public interface HelloInterface {
      void print(@HelloAnno(value = "Meow") String value);
   }

   public @interface HelloAnno {
      String value();
   }

   public @interface HelloAnnoRec {
      HelloAnno value();
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

   public sealed class Shape permits Circle {}
   public final class Circle extends Shape {}

   public record Cat(String name, int age) {};
}
