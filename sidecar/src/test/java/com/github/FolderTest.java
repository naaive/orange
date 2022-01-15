package com.github;

import org.junit.jupiter.api.Test;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Collectors;

public class FolderTest {

    @Test
    public void t1() throws IOException {
        List<String> collect = Files.list(Path.of("/")).map(x->x.toAbsolutePath().toString()).collect(Collectors.toList());


        System.out.println(collect);
    }
}

