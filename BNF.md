# Backus naur form for pseudocu to define sintax

&lt;natural&gt; ::= &lt;digit&gt; | &lt;nonzero&gt; &lt;natural&gt;

&lt;digit&gt; ::= **0** | &lt;nonzero&gt;

&lt;nonzero&gt; ::= **1** | **2** | **3** | **4** | **5** | **6** | **7** | **8** | **9**

&lt;letter&gt; ::=  
**a** | **b** | **c** | **d** | **e** | **f** | **g** | **h** | **i** | **j** | **k** | **l** | **m** | **n** | **o** | **p** | **q** | **r** | **s** | **t** | **u** | **v** | **w** | **x** | **y** | **z** |  
**A** | **B** | **C** | **D** | **E** | **F** | **G** | **H** | **I** | **J** | **K** | **L** | **M** | **N** | **O** | **P** | **Q** | **R** | **S** | **T** | **U** | **V** | **W** | **X** | **Y** | **Z**

&lt;variable&gt; ::= &lt;letter&gt; 
            | **_** + &lt;letter&gt; 
            | **_** + &lt;letter&gt; &lt;variable&gt; 
            | &lt;letter&gt; &lt;variable&gt; 
            | &lt;variable&gt; &lt;digit&gt;

&lt;arithmetic_op&gt; ::= **+** 
                | **-** 
                | ***** 
                | **/**

&lt;comparison_op&gt; ::= **!=** 
                | **==** 
                | **&gt;** 
                | **&lt;** 
                | **&gt;=** 
                | **&lt;=**

&lt;assignment_op&gt; ::= **=**

&lt;operator&gt; ::= &lt;arithmetic_op&gt; 
            | &lt;comparison_op&gt; 
            | &lt;assignment_op&gt;

# 1. Summarize the line rule further 
# 2. Operator hierarchy is missing

&lt;line&gt; ::= &lt;variable&gt; &lt;assignment_op&gt; &lt;variable&gt; 
    | &lt;variable&gt; &lt;assignment_op&gt; &lt;natural&gt;
    | &lt;variable&gt; &lt;assignment_op&gt; &lt;letter&gt;
    | &lt;variable&gt; &lt;assignment_op&gt; &lt;variable&gt; &lt;arithmetic_op&gt; &lt;natural&gt;
    | &lt;variable&gt; &lt;assignment_op&gt; &lt;variable&gt; &lt;arithmetic_op&gt; &lt;variable&gt;
    | &lt;variable&gt; &lt;assignment_op&gt; &lt;natural&gt; &lt;arithmetic_op&gt; &lt;natural&gt;