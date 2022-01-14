//package org.apache.lucene.store;
//
//import com.oracle.svm.core.annotate.Alias;
//import com.oracle.svm.core.annotate.RecomputeFieldValue;
//import com.oracle.svm.core.annotate.Substitute;
//import com.oracle.svm.core.annotate.TargetClass;
//
//@TargetClass(MMapDirectory.class)
//final class MMapDirectorySubstitutions {
//    @Alias
//    @RecomputeFieldValue(kind = RecomputeFieldValue.Kind.FromAlias, isFinal = true)
//    private static ByteBufferGuard.BufferCleaner CLEANER = null;
//
//    @Alias
//    @RecomputeFieldValue(kind = RecomputeFieldValue.Kind.FromAlias, isFinal = true)
//    private static boolean UNMAP_SUPPORTED = false;
//
//    @Substitute
//    private static Object unmapHackImpl() {
//        return "Manually disabled for graalvm";
//    }
//}