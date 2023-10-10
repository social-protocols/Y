# Entropy Ranking Formula Summary

Our ranking formula is based on the **Information Rate**. This is the expected information gained (or entropy eliminated) per unit of attention directed to a post.

The formula is:

	- for new impressions without note: s * q * lg(q/0.5)
	- for new impressions with note: r * p * lg(p/0.5) + t
	- for remedial impressions with note: s * ( p*lg(p/q) + (1-p)*lg((1-p)/(1-q)) ) + t

where:
	- p is the informed probability: probability of upvoting post when shown top note (if any)
	- q is the uninformed probability: probability of upvoting without being shown any note
	- r is the expected informed vote rate: expected votes/attention, when shown this note (if any)
	- s is the expected uninformed vote rate: expected votes/attention, when not shown any note
	- t is the information rate of note (calculated recursively)

## Notes

- A **remedial impressions** is when a post is shown to users who have already voted on the post without the note, because it is intended to remedy cognitive dissonance caused by users upvotes differing from what they would be if they were more informed.

- All these numbers are calculated using Bayesian averaging. If there are a large number of votes:
	p and q are approximately equal to historical upvotes/(upvotes + downvotes)
	r and s are approximately equal to historical (upvotes + downvotes) / attention

- Note p is the probability when shown the **top** note (whatever it is) where r is the expected vote rate when showing a post with **whatever note it is shown with** (which may not always be the top note).

- For new votes, s ≈ (upvotes + downvotes)/attention, q ≈ upvotes/(upvotes + downvotes), so:

	informationRate ≈ upvotes/attention * (log(upvotes / (upvotes + downvotes)) + 1) 


## Examples

### Example 1: New post with positive information gain rate

Suppose:
	- a post with no notes is given 500 attentionUnits worth of attention
	- receives 1000 total votes: 900 upvotes and 100 downvotes

Then:
	- q = Bayesian average global prior of 85% and 900/1000 ≈ .9
	- expected uninformed vote rate s = 1000/5000 = 2 votes / attentionUnit
	- expected informationRate = s*q*lg(q/0.5) = 2 * .9 * lg(.9/0.5) = 1.526 bits / attentionUnit


### Example 2: Post with negative information gain rate after note

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
		- uninformed probability for note A₂ is q₁ ≈ 9/10 
		- uninformed vote rate for note A₂ (when shone as note for post A) is s₁ ≈ 10/200
		- expected information rate for note A₂: s₁ * q*lg(q/0.5) ≈ .05 * .9 * lg(.9/.5) ≈ .038 bits / attentionUnit

	- so expected informationRate =  -.132 + t = - .094 bits / attentionUnit 

#### Discussion

In this example, showing a user the post has a negative information gain rate, even with the note. This is because for some people the post is new information, which 20% of people accept (believe) despite the note, and unreliable information increases entropy.

Contrast this to the example below, where we show the note to people who have already seen the post, in which case only the note is new information.

### Example 3: Remedial Impressions
	- Same post as above. But now the 1000 users who voted before being shown the note are shown the post with the note
	- new informed probability is still: p ≈ 20/100 ≈ .2
	- uninformed probability is still q ≈ .9
	- expected uninformed vote rate s = 1000/5000 = 2 votes / attentionUnit
	- expected informationRate: 2 * ( .2*lg(.2/.9) + .8*lg(.8/.1) ) + t = 3.93 + .038 = 4.31 bits / attentionUnit

### Discussion

In this example, the relatively high information rate is due to the large difference between p and q, creating a large amount of cognitive dissonance -- difference between what users believe and what they would believe if they saw the note. This cognitive dissonance is simply the potential information gain by showing the post to all these users.

Note also that not all users will change their vote. For each unit of attention, only a certain fraction of users actually "absorb" the post as new information, and of those only a certain fraction vote. But given the same amount of attention, we expected the same fraction of users to absorb the information whether or not they previously voted. We do expect some "stickiness", where users are resistant to change their beliefs or, more likely, to change their vote. So we would expect a decreased voteRate for remedial impressions. But the rate of information absorption should still be the same as it was for new users, which is the uninformed vote rate s.

Also the formula for remedial impressions assumes we are showing the post+note to users who already **voted** on the post, not just those who have been "shown" the post. Again, only if the user has voted do we know what their actual beliefs are. However, if users have seen the note and not voted, we know some fraction of those users "absorbed" the information and didn't vote. If we know what this fraction is, we can estimate the information gain rate for users who have already seen the post. This could be valuable, because a post+note might have negative information rate for users who haven't seen it (if p < 0.5, lg(p/0.5) is negative), but for users who have already absorbed the information in the note, the post will always positive information rate (KL divergence is always positive).




