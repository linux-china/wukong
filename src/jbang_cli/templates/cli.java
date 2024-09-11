///usr/bin/env jbang "$0" "$@" ; exit $?
//DEPS info.picocli:picocli:4.7.6

import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.Parameters;

import java.util.concurrent.Callable;

@Command(name = "{{className}}", mixinStandardHelpOptions = true, version = "{{className}} 0.1.0",
        description = "{{className}} made with JBang")
public class {{className}} implements Callable<Integer> {

    @Parameters(index = "0", description = "The greeting to print", defaultValue = "World!")
    private String greeting;

    public static void main(String... args) {
        int exitCode = new CommandLine(new {baseName}()).execute(args);
        System.exit(exitCode);
    }

    @Override
    public Integer call() throws Exception {
       // your business logic goes here...
       System.out.println("Hello " + greeting);
       return 0;
    }
}
