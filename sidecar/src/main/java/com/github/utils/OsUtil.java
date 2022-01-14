package com.github.utils;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Optional;

public class OsUtil {
    private static final String OS = System.getProperty("os.name").toLowerCase();

    public static Optional<Integer> winCpuUsage() {
        try {
            Process process = Runtime.getRuntime().exec("wmic cpu get loadpercentage");
            BufferedReader lineReader = new BufferedReader(new InputStreamReader(process.getInputStream()));
            @SuppressWarnings("OptionalGetWithoutIsPresent")
            String s = lineReader.lines().skip(2).findFirst().get();
            return Optional.of(Integer.valueOf(s));
        } catch (IOException e) {
            return Optional.empty();
        }
    }

    public static void main(String[] args) {

        System.out.println(OS);

        if (isWindows()) {
            System.out.println("This is Windows");
        } else if (isMac()) {
            System.out.println("This is MacOS");
        } else if (isUnix()) {
            System.out.println("This is Unix or Linux");
        } else if (isSolaris()) {
            System.out.println("This is Solaris");
        } else {
            System.out.println("Your OS is not supported!!");
        }
    }

    public static boolean isWindows() {
        return OS.contains("win");
    }

    public static boolean isMac() {
        return OS.contains("mac");
    }

    public static boolean isUnix() {
        return (OS.contains("nix") || OS.contains("nux") || OS.contains("aix"));
    }

    public static boolean isSolaris() {
        return OS.contains("sunos");
    }

    public static String getOS() {
        if (isWindows()) {
            return "win";
        } else if (isMac()) {
            return "osx";
        } else if (isUnix()) {
            return "uni";
        } else if (isSolaris()) {
            return "sol";
        } else {
            return "err";
        }
    }
}
