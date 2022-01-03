package com.github.accessor;

import org.junit.jupiter.api.Test;

import java.util.List;

class IndexAccessorTest {
    private static final String indexPath =
            "C:\\Users\\Administrator\\IdeaProjects\\orange\\src\\main\\resources\\.orange\\index";
    private static String dataPath =
            "C:\\Users\\Administrator\\IdeaProjects\\orange\\src\\main\\resources\\.orange\\data";
    @Test
    void search() {
        DbAccessor dbAccessor = new DbAccessor(dataPath);
        IndexAccessor indexAccessor = new IndexAccessor(indexPath,dbAccessor);
        List<FileView> chrome = indexAccessor.search("chrome");
        System.out.println(chrome);
    }
}
