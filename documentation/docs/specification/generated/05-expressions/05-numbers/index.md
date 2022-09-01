<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Numbers

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="NumericLiteralProduction"><span style="color: var(--md-code-hl-keyword-color);">«NumericLiteral»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-operator-color);">(</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalNumberProduction">«DecimalNumber»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#HexNumberProduction">«HexNumber»</a></span><span style="color: var(--md-code-hl-operator-color);">)</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#NumberUnitProduction">«NumberUnit»</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="DecimalNumberProduction"><span style="color: var(--md-code-hl-keyword-color);">«DecimalNumber»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-operator-color);">(</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalIntegerProduction">«DecimalInteger»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalFloatProduction">«DecimalFloat»</a></span><span style="color: var(--md-code-hl-operator-color);">)</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalExponentProduction">«DecimalExponent»</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="DecimalIntegerProduction"><span style="color: var(--md-code-hl-keyword-color);">«DecimalInteger»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">'0'</span><span style="color: var(--md-code-hl-operator-color);">…</span><span style="color: var(--md-code-hl-string-color);">'9'</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">{</span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-string-color);">'_'</span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">'0'</span><span style="color: var(--md-code-hl-operator-color);">…</span><span style="color: var(--md-code-hl-string-color);">'9'</span><span style="color: var(--md-code-hl-operator-color);">}</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="DecimalFloatProduction"><span style="color: var(--md-code-hl-keyword-color);">«DecimalFloat»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalIntegerProduction">«DecimalInteger»</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">'.'</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalIntegerProduction">«DecimalInteger»</a></span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="DecimalExponentProduction"><span style="color: var(--md-code-hl-keyword-color);">«DecimalExponent»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-operator-color);">(</span><span style="color: var(--md-code-hl-string-color);">'e'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'E'</span><span style="color: var(--md-code-hl-operator-color);">)</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-string-color);">'-'</span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#DecimalIntegerProduction">«DecimalInteger»</a></span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="HexNumberProduction"><span style="color: var(--md-code-hl-keyword-color);">«HexNumber»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"0x"</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/06-strings#HexCharacterProduction">«HexCharacter»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">{</span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-string-color);">'_'</span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/06-strings#HexCharacterProduction">«HexCharacter»</a></span><span style="color: var(--md-code-hl-operator-color);">}</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="NumberUnitProduction"><span style="color: var(--md-code-hl-keyword-color);">«NumberUnit»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"days"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"ether"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"finney"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"gwei"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"hours"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"minutes"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"seconds"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"szabo"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"weeks"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"wei"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"years"</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/05-expressions/05-numbers/index.md"