# Solution notes: Reverse the word order

## Approach

_first i had created a vector for storing the String type words in the char_vec and the res is for getting and stroring the idivsual words and then the ownership of res is moved to char_vec one element after than all words are got than the vec is traversed from the last to the first index and the words are stored with a space in there right side in the res and then the reverse word is returned_

## Edge cases handled
- _for handeling collapse_inner_whitespace() ihad checked that if the word is not "" then its added to final string else not_
-  _for handling the tabs_and_newlines_count_as_whitespace() i had used is_whitespace() which checks for space as well as for escape Sequence_

## Anything special

_The main trick was using the is_whitespace()_
