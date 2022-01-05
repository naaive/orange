package com.github.accessor;

import org.junit.jupiter.api.Test;

import java.io.File;
import java.util.Arrays;
import java.util.List;

class IndexAccessorTest {

    @Test
    public void t1() {
        File[] files = File.listRoots();
        System.out.println(Arrays.toString(files));
    }
    private static final String indexPath =
            "C:\\Users\\Administrator\\IdeaProjects\\orange\\src\\main\\resources\\.orange\\index";
    private static String dataPath =
            "C:\\Users\\Administrator\\IdeaProjects\\orange\\src\\main\\resources\\.orange\\data";
    @Test
    void search() {
        DbAccessor dbAccessor = new DbAccessor(dataPath);
        IndexAccessor indexAccessor = new IndexAccessor(indexPath,dbAccessor, null);
        FileDoc fileDoc = new FileDoc();
        fileDoc.setName("jeff hello wrold");
        fileDoc.setExt("hi");
        fileDoc.setAbsPath("jeff/hello/wrold");
        fileDoc.setSize(1L);
        fileDoc.setCreatedAt(12312L);
        fileDoc.setModifiedAt(231L);
//        indexAccessor.add(fileDoc);

        List<FileView> chrome = indexAccessor.search("index");
        System.out.println(chrome);
    }

    @Test
    void suggest() {


    }
}
