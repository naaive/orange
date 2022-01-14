//package org.apache.lucene.analysis;
//
//
//import com.oracle.svm.core.annotate.Alias;
//import com.oracle.svm.core.annotate.RecomputeFieldValue;
//import com.oracle.svm.core.annotate.TargetClass;
//import org.apache.lucene.analysis.tokenattributes.PackedTokenAttributeImpl;
//import org.apache.lucene.util.AttributeFactory;
//import org.apache.lucene.util.AttributeFactoryHelpers;
//
//@TargetClass(TokenStream.class)
//final class TokenStreamSubstitutions {
//    @Alias
//    @RecomputeFieldValue(kind = RecomputeFieldValue.Kind.FromAlias, isFinal = true)
//    public static AttributeFactory DEFAULT_TOKEN_ATTRIBUTE_FACTORY = AttributeFactoryHelpers.getStaticImplementation(AttributeFactory.DEFAULT_ATTRIBUTE_FACTORY, PackedTokenAttributeImpl.class);
//}