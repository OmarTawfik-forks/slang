<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 1.4. Using Directives

```{ .ebnf #UsingDirective }

```

<pre ebnf-snippet="UsingDirective" style="display: none;"><a href="#UsingDirective"><span class="k">UsingDirective</span></a><span class="o"> = </span><span class="cm">(* using_keyword: *)</span><span class="o"> </span><a href="../06-keywords#UsingKeyword"><span class="k">USING_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* clause: *)</span><span class="o"> </span><a href="#UsingClause"><span class="k">UsingClause</span></a><br /><span class="o">                 </span><span class="cm">(* for_keyword: *)</span><span class="o"> </span><a href="../06-keywords#ForKeyword"><span class="k">FOR_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* target: *)</span><span class="o"> </span><a href="#UsingTarget"><span class="k">UsingTarget</span></a><br /><span class="o">                 </span><span class="cm">(* global_keyword: *)</span><span class="o"> </span><a href="../06-keywords#GlobalKeyword"><span class="k">GLOBAL_KEYWORD</span></a><span class="o">?</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.13 *)</span><br /><span class="o">                 </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #UsingClause }

```

<pre ebnf-snippet="UsingClause" style="display: none;"><a href="#UsingClause"><span class="k">UsingClause</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">            | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#UsingDeconstruction"><span class="k">UsingDeconstruction</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.13 *)</span></pre>

```{ .ebnf #UsingDeconstruction }

```

<pre ebnf-snippet="UsingDeconstruction" style="display: none;"><span class="cm">(* Introduced in 0.8.13 *)</span><br /><a href="#UsingDeconstruction"><span class="k">UsingDeconstruction</span></a><span class="o"> = </span><span class="cm">(* open_brace: *)</span><span class="o"> </span><a href="../07-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                      </span><span class="cm">(* symbols: *)</span><span class="o"> </span><a href="#UsingDeconstructionSymbols"><span class="k">UsingDeconstructionSymbols</span></a><br /><span class="o">                      </span><span class="cm">(* close_brace: *)</span><span class="o"> </span><a href="../07-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #UsingDeconstructionSymbols }

```

<pre ebnf-snippet="UsingDeconstructionSymbols" style="display: none;"><span class="cm">(* Introduced in 0.8.13 *)</span><br /><a href="#UsingDeconstructionSymbols"><span class="k">UsingDeconstructionSymbols</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#UsingDeconstructionSymbol"><span class="k">UsingDeconstructionSymbol</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#UsingDeconstructionSymbol"><span class="k">UsingDeconstructionSymbol</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #UsingDeconstructionSymbol }

```

<pre ebnf-snippet="UsingDeconstructionSymbol" style="display: none;"><span class="cm">(* Introduced in 0.8.13 *)</span><br /><a href="#UsingDeconstructionSymbol"><span class="k">UsingDeconstructionSymbol</span></a><span class="o"> = </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">                            </span><span class="cm">(* alias: *)</span><span class="o"> </span><a href="#UsingAlias"><span class="k">UsingAlias</span></a><span class="o">?</span><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.19 *)</span></pre>

```{ .ebnf #UsingAlias }

```

<pre ebnf-snippet="UsingAlias" style="display: none;"><span class="cm">(* Introduced in 0.8.19 *)</span><br /><a href="#UsingAlias"><span class="k">UsingAlias</span></a><span class="o"> = </span><span class="cm">(* as_keyword: *)</span><span class="o"> </span><a href="../06-keywords#AsKeyword"><span class="k">AS_KEYWORD</span></a><br /><span class="o">             </span><span class="cm">(* operator: *)</span><span class="o"> </span><a href="#UsingOperator"><span class="k">UsingOperator</span></a><span class="o">;</span></pre>

```{ .ebnf #UsingOperator }

```

<pre ebnf-snippet="UsingOperator" style="display: none;"><span class="cm">(* Introduced in 0.8.19 *)</span><br /><a href="#UsingOperator"><span class="k">UsingOperator</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Ampersand"><span class="k">AMPERSAND</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Asterisk"><span class="k">ASTERISK</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#BangEqual"><span class="k">BANG_EQUAL</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Bar"><span class="k">BAR</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Caret"><span class="k">CARET</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#EqualEqual"><span class="k">EQUAL_EQUAL</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#GreaterThan"><span class="k">GREATER_THAN</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#GreaterThanEqual"><span class="k">GREATER_THAN_EQUAL</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#LessThan"><span class="k">LESS_THAN</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#LessThanEqual"><span class="k">LESS_THAN_EQUAL</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Minus"><span class="k">MINUS</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Percent"><span class="k">PERCENT</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Plus"><span class="k">PLUS</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Slash"><span class="k">SLASH</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Tilde"><span class="k">TILDE</span></a><span class="o">;</span></pre>

```{ .ebnf #UsingTarget }

```

<pre ebnf-snippet="UsingTarget" style="display: none;"><a href="#UsingTarget"><span class="k">UsingTarget</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">            | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Asterisk"><span class="k">ASTERISK</span></a><span class="o">;</span></pre>
