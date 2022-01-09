package com.github.accessor;

import com.github.conf.IndexConf;
import com.github.ik.IKAnalyzer;
import io.netty.channel.DefaultEventLoopGroup;
import lombok.extern.java.Log;
import org.apache.lucene.search.suggest.analyzing.AnalyzingInfixSuggester;
import org.apache.lucene.store.FSDirectory;
import org.apache.lucene.util.BytesRef;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.stream.Collectors;

@Log
public class FileDocSuggester {

    private AnalyzingInfixSuggester suggester;
    private static final int COMMIT_THRESHOLD = 10000;
    private int addCnt = 0;
    private final DefaultEventLoopGroup executors;

    public FileDocSuggester(DefaultEventLoopGroup executors) {
        this.executors = executors;

        initialize();
    }

    private void initialize() {
        executors.scheduleAtFixedRate(
                () -> {
                    log.info(String.format("refresh %s keyword(s) to index", addCnt));
                    addCnt = 0;
                    try {
                        suggester.refresh();
                    } catch (IOException e) {
                        log.log(Level.SEVERE, "refresh suggester err", e);
                    }
                },
                5,
                5,
                TimeUnit.SECONDS);
        try {

            FSDirectory directory = FSDirectory.open(Paths.get(IndexConf.SUGGEST_CONF));
            suggester = new AnalyzingInfixSuggester(directory, new IKAnalyzer(false));
            suggester.build(new FileDocIterator(Collections.emptyList()));
        } catch (IOException e) {
            log.log(Level.SEVERE, "FileDocSuggester initialize err", e);
        }
    }

    public void put(String name) {
        try {
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
                log.info(String.format("refresh %s keyword(s) to index", addCnt));
                addCnt = 0;
            }
        } catch (IOException e) {
            log.log(Level.SEVERE, "put err", e);
        }
    }

    public List<String> lookup(String key) {
        try {
            return suggester.lookup(key, 6, true, false).stream()
                    .map(x -> x.key.toString())
                    .collect(Collectors.toList());
        } catch (IOException e) {
            log.log(Level.SEVERE, "lookup err", e);
        }
        return Collections.emptyList();
    }
}
