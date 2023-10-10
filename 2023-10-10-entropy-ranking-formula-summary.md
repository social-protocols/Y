# Entropy Ranking Formula Summary

Our ranking formula is based on the **Information Rate**. This is the expected information gained (or entropy eliminated) per unit of attention directed to a post.

The formula is:

- for new impressions without note: s * q * lg(q/0.5)
- for new impressions with note: r * p * lg(p/0.5) + t
- for remedial impressions with note: s * Dkl(p || q) + t

where:

- p is the informed probability: probability of upvote on post when shown top note (if any)
- q is the uninformed probability: probability of upvote on note without being shown any note
- r is the expected informed vote rate: expected votes/attention, when shown this note (if any)
- s is the expected uninformed vote rate: expected votes/attention, when not shown any note
- t is the information rate of note (calculated recursively)
- Dkl is KL divergence, or relative entropy: 
	- Dkl(p || q) = p*lg(p/q) + (1-p)*lg((1-p)/(1-q))

## Notes

- A **remedial impression** is when a post is shown to users who have already voted on the post without the note, because it is intended to remedy cognitive dissonance caused by users upvotes differing from what they would be if they saw the note. A **new impression** is an impression for users who haven't been exposed to the post.

- Note p is the probability when shown the **top** note (whatever it is) where r is the expected vote rate when showing a post with **whatever note it is shown with** (which may not always be the top note).

- All these numbers are calculated using Bayesian averaging. If there are a large number of votes:
	- p and q are approximately equal to historical upvotes/(upvotes + downvotes)
	- r and s are approximately equal to historical (upvotes + downvotes) / attention

- Thus for new impressions, informationRate ≈ upvotes/attention * (log(upvotes / (upvotes + downvotes)) + 1) 

## Examples

### Example 1: New post with positive information rate

Suppose:
- a post with no notes is given 500 attentionUnits worth of attention
- receives 1000 total votes: 900 upvotes and 100 downvotes

Then:
- q = Bayesian average global prior of 85% and 900/1000 ≈ .9
- expected uninformed vote rate s = 1000/500 = 2 votes / attentionUnit
- expected informationRate = s * q * lg(q/0.5) = 2 * .9 * lg(.9/0.5) = 1.526 bits / attentionUnit


### Example 2: Post with negative information rate after note

Suppose:
- a post A with a note A₂ is given additional 200 attentionUnits worth of attention	 
- post A received 100 additional votes: 20 upvotes and 80 downvotes
- note A₂ on post A received 10 votes, 9 of which were upvotes.
- note A₂ has no subnotes

Then:
- new informed probability for post A is p ≈ 20/100 ≈ .2
- uninformed probability is still q ≈ .9
- expected informed vote rate r = 100/200 = 1/2 votes / attentionUnit
- expected informationRate = r * p*lg(p/0.5) + t = 1/2 * .2 * lg(.2/0.5) + t = -.132 + t bits / attentionUnit 

- It remains to calculate t, the information rate for the note itself:
	- uninformed probability for note A₂ is q₂ ≈ 9/10 
	- uninformed vote rate for note A₂ (when shone as note for post A) is s₂ ≈ 10/200 ≈ .05
	- expected information rate for note A₂: s₂ * q*lg(q₂/0.5) ≈ .05 * .9 * lg(.9/.5) ≈ .038 bits / attentionUnit

- so expected informationRate for A is =  -.132 + t = - .094 bits / attentionUnit 

#### Discussion

In this example, showing a user the post has a negative information rate, even with the note. This is because for some people the post is new information, which 20% of people accept (believe) despite the note, and acceptance of unreliable information increases entropy.

Contrast this to the example below, where we show the note only to people who have already voted on the post, in which case only the note is new information, and the information rate is therefore positive.

### Example 3: Remedial Impressions

Suppose:

- Same post as above. But now the 1000 users who voted before being shown the note are shown the post with the note

Then:
- new informed probability is still: p ≈ 20/100 ≈ .2
- uninformed probability is still q ≈ .9
- expected uninformed vote rate is still s = 1000/5000 = 2 votes / attentionUnit
- expected informationRate: 2 * ( .2*lg(.2/.9) + .8*lg(.8/.1) ) + .038 = 3.93 + .038 = 3.97 bits / attentionUnit

#### Discussion

In this example, the relatively high information rate is due to the large difference between p and q, creating a large amount of cognitive dissonance -- difference between what users believe and what they would believe if they saw the note. This cognitive dissonance is simply the potential information gain by showing the post to all these users.

Note also that not all users will change their vote. For each unit of attention, only a certain fraction of users actually "absorb" the post as new information, and of those only a certain fraction vote. But given the same amount of attention, we expected the same fraction of users to absorb the information whether or not they previously voted. 

