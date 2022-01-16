package com.github.accessor;

import com.github.conf.AppConf;
import com.github.ik.IKAnalyzer;
import io.netty.channel.DefaultEventLoopGroup;
import lombok.extern.java.Log;
import lombok.val;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.search.spell.JaroWinklerDistance;
import org.apache.lucene.search.spell.LuceneDictionary;
import org.apache.lucene.search.spell.SpellChecker;
import org.apache.lucene.search.suggest.analyzing.AnalyzingInfixSuggester;
import org.apache.lucene.store.FSDirectory;
import org.apache.lucene.util.BytesRef;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
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
    private volatile SpellChecker spellchecker;

    public FileDocSuggester(DefaultEventLoopGroup executors) {
        this.executors = executors;

        log.info("init FileDocSuggester");
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

            FSDirectory directory = FSDirectory.open(Paths.get(AppConf.SUGGEST_CONF));
            suggester = new AnalyzingInfixSuggester(
                    directory, new IKAnalyzer(false), new IKAnalyzer(true), 1, true, false, false);
            suggester.build(new FileDocIterator(Collections.emptyList()));

        } catch (IOException e) {
            log.log(Level.SEVERE, "FileDocSuggester initialize err", e);
        }

        executors.scheduleAtFixedRate(
                this::rebuildSpellChecker,
                5,
                60,
                TimeUnit.MINUTES);
    }

    private void rebuildSpellChecker() {
        try {
            FSDirectory indexDir = FSDirectory.open(Paths.get(AppConf.INDEX_PATH));
            DirectoryReader open = DirectoryReader.open(indexDir);
            SpellChecker spellchecker = new SpellChecker(indexDir);
            LuceneDictionary absPath = new LuceneDictionary(open, FileDoc.NAME);
            spellchecker.indexDictionary(absPath,new IndexWriterConfig(),true);
            spellchecker.setStringDistance(new JaroWinklerDistance());
            open.close();

            if (this.spellchecker != null) {
                this.spellchecker.close();
            }
            this.spellchecker = spellchecker;
        } catch (IOException e) {
            log.log(Level.SEVERE,"failed to rebuild spell checker");
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
            List<String> collated = suggester.lookup(key, 6, true, false).stream()
                    .map(x -> x.key.toString())
                    .collect(Collectors.toList());
            if (collated.isEmpty()) {
                collated = spellCheck(key);
            }
            return collated;
        } catch (IOException e) {
            log.log(Level.SEVERE, "lookup err", e);
        }
        return Collections.emptyList();
    }

    /**
     * called when no suggestions
     * @param key
     * @return
     * @throws IOException
     */
    private List<String> spellCheck(String key) throws IOException {
        return Arrays.stream(spellchecker.suggestSimilar(key, 6)).collect(Collectors.toList());
    }
}
