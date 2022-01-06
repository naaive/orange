package com.github.accessor;

import com.github.conf.IndexConf;
import com.github.ik.IKAnalyzer;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.apache.lucene.search.suggest.analyzing.AnalyzingInfixSuggester;
import org.apache.lucene.store.FSDirectory;
import org.apache.lucene.util.BytesRef;

import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

@Slf4j
public class FileDocSuggester {

    private AnalyzingInfixSuggester suggester;
    private static final int COMMIT_THRESHOLD = 10000;
    private int addCnt = 0;

    public FileDocSuggester() {

        initialize();
    }

    @SneakyThrows
    private void initialize() {
        FSDirectory directory = FSDirectory.open(Paths.get(IndexConf.SUGGEST_CONF));
        suggester = new AnalyzingInfixSuggester(directory, new IKAnalyzer(false));
        suggester.build(new FileDocIterator(Collections.emptyList()));
    }

    @SneakyThrows
    public void put(String name) {
        ArrayList<FileDoc> fileDocs = new ArrayList<>();
        fileDocs.add(new FileDoc().setName(name));
        FileDocIterator iter = new FileDocIterator(fileDocs);

        while (iter.hasPayloads()) {
            BytesRef next = iter.next();
            suggester.update(next, Collections.emptySet(), iter.weight(), iter.payload());
            addCnt++;
        }
        if (addCnt % COMMIT_THRESHOLD == 0) {
            suggester.refresh();
            log.info("refresh {} keyword(s) to index", addCnt);
            addCnt = 0;
        }
    }

    @SneakyThrows
    public List<String> lookup(String key) {
        return suggester.lookup(key, 6, true, false).stream()
                .map(x -> x.key.toString())
                .collect(Collectors.toList());
    }
}
