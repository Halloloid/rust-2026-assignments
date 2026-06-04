# Solution notes: Longest word slice

## Approach

_Intialized **Two Tuples** for storing the Start and End Index , in index 0 the Longest Size of Word and is stored and in the 1 index i store the index right now iterating two cases when there is a space one word is completed so checked for longest and swapped and intially added a **extra space** at the **end** for getting the last word_

## Edge cases handled
- _Added a Space at last to handel single character and Word_
- _at last checked if its Some("") then i return None for Handeling the Empty and Only whitsapce Case_


## Anything special

_One Trick Came to mind is Using 2 Elemet Size of Tuple for Storing index and Adding One Space at End_
