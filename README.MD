Day 38: Burrows-Wheeler
<br>
The Burrows–Wheeler transform is an algorithm used to prepare data for use with data compression techniques such as bzip2. It was invented by Michael Burrows and David Wheeler in 1994 while Burrows was working at DEC Systems Research Center in Palo Alto, California. It is based on a previously unpublished transformation discovered by Wheeler in 1983. The algorithm can be implemented efficiently using a suffix array thus reaching linear time complexity.[1]
<br>
When a character string is transformed by the BWT, the transformation permutes the order of the characters. If the original string had several substrings that occurred often, then the transformed string will have several places where a single character is repeated multiple times in a row.
<br>
For example:
<br>
```
Input	SIX.MIXED.PIXIES.SIFT.SIXTY.PIXIE.DUST.BOXES
Output	TEXYDST.E.IXIXIXXSSMPPS.B..E.S.EUSFXDIIOIIIT[2]
```
The output is easier to compress because it has many repeated characters. In this example the transformed string, there are a total of eight runs of identical characters: XX, II, XX, SS, PP, .., II, and III, which together make 17 out of the 44 characters.
<br>
run:
```
fn main() {
    let input = "the theta, there and there, was her";
    let (target, index) = bwt(input);
    let target: String = target.into_iter().collect();
    assert_eq!(target, "es,de,aet wnrhrhhhhttt taeeeaer    ");
    let source = ibwt(input, index);
    let inverse_target: String = source.into_iter().collect();
    assert_eq!(inverse_target, input);
}
```