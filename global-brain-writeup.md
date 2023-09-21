# The Global Brain

> *Falsehood flies, and truth comes limping after it, so that when men come to be undeceived, it is too late...*
>
> -- Thomas Swift

## Intro

The Global Brain algorithm converts a social network into a distributed brain, where groups of individuals act like neurons that process information and pass it on to others, resulting in a whole that is more intelligent than the individual human parts.

The algorithm works by identifying information that is likely to change minds, and modeling how those changes propagate through the network. It then uses this information to focus attention on posts that reduce "cognitive dissonance" -- difference of opinion due to people being exposed to different information.

The result is [social protocol] that drives productive conversations, maximizes the flow of useful and reliable information, reduces the flow of misinformation, and increases [human alignment](https://social-protocols.org/human-alignment-technology/).

### Informed Votes

The basic computational unit in the Global Brain is the **informed vote**. Before users vote on post, the UI shows them a **note** (any reply to that post), and then records their vote **given** they were shown that note before voting. If the informed vote is different from the uninformed vote, it means the note **caused** changes in votes. We call notes that do change behavior **informative**. 

TODO: mention Community Notes. Distributed Bayesian Reasoning

### The Causal Model

So every reply to every post becomes effectively an AB test that tells us how exposure to a note effects votes on a post. And one thing that makes properly-run AB tests very powerful is that they establish **causality**. 

Establishing causal links allows us to model the Global Brain as a causal Bayesian network. The causal links between notes and posts are the synapses of the Global Brain. They allow us to predict how a random user **would** vote if they were exposed to some combination of posts, by predicting how how exposure to those posts will influence upvotes on other posts, and how that influences upvotes on other posts, and so on. 

### Distributed Reasoning

By simulating how beliefs propagates through the network, the global brain engages in a form of **reasoning**, loosely defined. Each neuron in the global brain reasons -- or processes information -- using the same inferences the average user makes. But unlike the average user, the Global Brain can process **all** information in the system in a valid Bayesian manner. So in a way the global simulates an average user who has no unlimited time, memory, and processing capacity.

We can then query this model to estimate how the Global Brain would vote if it was exposed to **all** information -- all posts ever made. We can also use the model to identify where to direct users' attention in order to reduce cognitive dissonance in the network.

### Informal Argument Model 

The Global Brain does not require a formal model of belief. It doesn't need posts to be structured as formal propositions or claims. It doesn't need to know if users agree or disagree with posts.

The only assumptions are that 1) a vote reflects a users intent to give a post more/less attention (see [The Law of Attention](https://jonathanwarden.com/the-law-of-attention)) and 2) this intent is caused by underlying beliefs that we cannot observe (latent variables -- see section below).

But by making some reasonable assumptions about the causal relationship between beliefs and votes, then by watching how exposure to notes causes changes to votes, we can model these underlying beliefs and predict how exposure to a note will cause changes to these beliefs, which will cause changes to other beliefs, and so on. We described our model in more detail below.

#### EXAMPLE

HERE: Example showing how more information converges on truth
Making hypothetical predictions about what users **would** believe is valuable because no user has been exposed to all combinations of notes. For example, someone may reply A (a lie) with note B (another lie), ....etc. etc.


### Reason and Argument

A key assumption we make is that more information leads to better judgments. Even though people can post false and misleading information, people can respond by explaining why this information is false or misleading. Even though people can promote hateful or harmful ideas, people can respond with reasons why these these ideas are wrong.

By identifying information that changes minds, driving attention to that information, and then identifying responses that change minds back, the system drives informative conversations. And although this isn't guaranteed to lead to the truth, it will at least result in more informed users, and a estimate of what of how users upvote after being exposed to all the strongest information on arguments on both sides.

[Wnat to hear what people have to say and judge for ourselves]

In the example above, once the model learns that B is changing people's votes on A, it directs more attention to B, resulting in fewer upvotes for A. We can think of B as an argument against A, or a reason not to upvote A. The "reason" doesn't need to take any particular logical or rhetorical form. It is a reason by virtue of being the literal reason that people change their upvotes -- the  observed cause of changes to upvotes.

Unfortunately, in the example above, B was a lie. But fortunately, C is a convincing reason to believe that C is a lie. C actually causes people **not** to change their vote on A, in spite of being exposed to B. B and C cancel each other out, so to speak.

Now, the algorithm knows that, first, it is no longer necessary to direct attention to B, because it knows that it would not change votes among users who were also informed about C. Second, it knows that it needs to "undo" the harm done by the misinformation in B,  directing attention of users who were exposed to B to the counter-argument C.

The metric used for measuring cognitive dissonance and using that to determine where to direct attention is described more below.

TOOD: adversarial process

todo: deliberative poll

### Checking Misinformation

Misinformation is harmful in social networks only when it is **unchecked**. Lies are only harmful if people don't see the responses to the lies.

The engagement-based algorithms in today's social networks encourage the unchecked spread of rumors (with the exception of Twitter with Community Notes), regardless of how well they are supported, wheres the Global Brain algorithm discourages the spread of information that people are unlikely to upvote if they were fully informed, and encourages the spread of information that might check the spread of a false information that is already spreading.


[Todo: Thomas Swift said "falsehood flies...", but the global brain ensure that truth flies after falsehood.]

And yet the purpose of the Global Brain algorithm is NOT to tell people what is true. It is simply to direct people's attention to information that may change their votes.

But why do we want to change votes? That sounds sinister. Propaganda changes how people vote. But propaganda works by selectively exposing users to information with the goal of changing opinions about specific things, all the while actively omitting contrary information. It is one sided and dishonest. The Global Brain, on the other hand, has no agenda other than driving productive conversation and reducing cognitive dissonance.

### Marketplace of Ideas

So the Global Brain algorithm, drives a fair, unbiased process of weighing all the arguments that anyone cares to make. As long as there is sufficient intellectual diversity among recipients, the result is an adversarial process, where all the relevant information (the posts that change minds either way) is exposed and processed. (the marketplace of ideas).


## Defining Cognitive Dissonance

The Global Brain only requires a small number of people to actually change their minds in order to learn how beliefs effect other beliefs and construct the causal Bayesian network. So the predictions of the model may differ significantly from most the beliefs of the majority.

For example, suppose 1000 users voted on a post, and most people upvoted it. Then suppose that among a subset of 30 users who saw some very informative note, the probability of upvoting the post dropped to close to zero. We can thus estimate that if all 1000 users saw that note, they would not have upvoted the post. Even though in actuality most users did upvote it.

In such situations, we say there is a large amount of cognitive dissonance in the network. Cognitive dissonance arises whenever people's upvotes differe from what their upvotes **would** be if they were exposed to all the relevant information. Or in other words, when peoples votes differ from the predictions of the causal model.

The goal of the Global Brain algorithm is minimizing cognitive dissonance. It does this by exposing users to the informative note, reducing the inconsistency between what people upvote and what they would upvote if they were more informed.

Reducing cognitive dissonance brings individual participants into greater alignment, reducing differences of opinion due to differences in information. It cannot of course eliminate differences of opinion, because opinion can be subjective, because people have different priors and different ways of processing information. But it can identify situations where differences are due to ignorance of readily available information: when one group of people believe something that another group doesn't only because they haven't seen the same posts.

The result is a network that learns. The model learns as new information posts are submitted to the network. And the human participants in the network also learn as they are exposed to posts that bring their mental models into greater alignment.


### Minimizing Cross Entropy

We can measure cognitive dissonance as the cross entropy between users actual beliefs and their hypothetical fully-informed beliefs (the beliefs they would have if exposed to all posts).

Interestingly, this measure looks very similar to a loss function commonly used when training a neural network. If 𝑦ᵢⱼ are the votes of users 𝑖 on post 𝑗, and 𝑦̂ⱼ is the estimated probability that users would upvote post 𝑗 given they were exposed to all relevant notes in the system, then the total "cognitive dissonance", or total cross entropy, is:

    - 1/m∑_i 1/n∑_j 𝑦ᵢⱼlog(𝑦̂ⱼ) + (1 - 𝑦ᵢⱼ)log(1 - 𝑦̂ⱼ)

But unlike a neural network, the Global Brain algorithm attempts to minimize cognitive dissonance not by changing its predictions (𝑦̂ⱼ), but by exposing users to notes that changes **their** beliefs (𝑦ᵢⱼ). 

A [social protocol] determines how the attention of the participants in the protocol is directed. So the Global Brain algorithm is trying to direct attention to posts that reduce cross entropy.

Optimizing for reducing cross entropy drives productive conversations. Consider again the above example, where subnote C exactly cancels the effect of note B. This means that, for users who have not been exposed to B, users vote as predicted. The average value of 𝑦ᵢⱼ for these users equals the fully-informed estimate 𝑦̂ⱼ. Cross-entropy is minimized when two probability distributions are equal (when (1/m ∑_i 𝑦̄ᵢⱼ) = 𝑦̂ⱼ), so it is impossible to further reduce cross-entropy for users who have not seen post B. Exposing users to B without exposing them to C can only increase cross-entropy! The algorithm determines, correctly, that driving more attention to B would not be productive.

However, since some users have already had their minds changed by B, their is a difference between their votes on posts A and B and the fully-informed estimate. So there is still cognitive dissonance among the subset of users exposed to B. Exposing that subset of users to post C, until the upvote percentage of that subset matches the overall upvote percentage, will result in minimal cross-entropy.


### CAUSAL MODEL

The global brain requires modeling users beliefs as a causal Bayesian network. But making this link is tricky, because we don't actually know what users believe, only how they vote. Suppose we know how much exposure to post B changed the probability that users will upvote post A, and how much exposure to post C changes the probability that a user will upvote post B. Can we predict how much exposure to post C will change the probability that a user will upvote post A?

We can draw a causal graph with our assumptions of how information **causes** changes in votes. We assume that votes are influenced by hidden variables, which are users' actual beliefs. For example, when somebody posts B, and it influences their vote on A, B must contain some information that users didn't already have, and that gives them a reason to change their vote. If B did not contain new information, why would it change their behavior?

Even a humorous comment can be modeled as information for our purposes. Suppose somebody responds to post A with a purely humorous comment B, and this causes more people to upvote A. Why would a joke cause more people to upvote A? Since we assume votes indicate intent to direct attention, it can only mean that the fact that joke B was funny caused people to believe that A should get more attention. Maybe it convinced them that A could be an amusing topic, for example. 

But we don't need to have a theory of how people reason or what it is in people's minds that cause them to change their behavior. It is sufficient to assert that there is some latent variable that links exposure to a post to voting behaviors. 

However, to understand this process it helps to think in purely Bayesian terms. So we say if something causes people to change votes, it is a **reason**. So we can say that these reasons are unobserved "underlying beliefs" in some proposition.

Let's say 𝓑 represents users' underlying belief that was directly changed by the post B (e.g. a B is video of bigfoot, which causes users to change their belief in 𝓑=*there is video evidence that bigfoot exists*). The belief in 𝓑 has a causal effect on votes on B. 𝓑 also has a causal effect on some other underlying belief 𝓐 (e.g. 𝓐=*bigfoot exists*). Finally, the underlying belief 𝓐 has a causal effect on upvotes on post A ("Bigfoot is real, people!")

Then let's use the term sB and sA to represent the event that users were exposed to posts B and A respectively. And finally we'll use italic 𝐵 and 𝐴 to represent upvotes on B and A respectively.

So showing a user post B (sB) effects belief in 𝓑 which effects votes 𝐵, and so on. And belief 𝓑 also effects belief 𝓐. So our causal graph looks like this.

    sB → 𝓑 → 𝐵
         ↓
    sA → 𝓐 → 𝐴

Now, let's assume that the number of upvotes 𝐴 or 𝐵 is directly proportional to the number of people that believe 𝓐 or 𝓑 respectively. That is, the more informative a post, the more likely users are to upvote it. 

    P(𝐴=1) ∝ P(𝓐=1)
    P(𝐵=1) ∝ P(𝓑=1)

Or more specifically:

    P(A=1) = c * P(𝓐=1)
    P(B=1) = d * P(𝓑=1)

So we can kind of use A and B as proxies for 𝓐 and 𝓑.

Now, suppose a user responds to post B with post C (e.g. "That video is a guy in a costume! You can see his watch at 32s"). **Seeing** post C (sC) indirectly causes a change in the probability that users upvote A. And for the moment, assume that sC *only* effects A through 𝓑 (that is, 𝓐 is conditionally independent of sC given 𝓑. We can relax this assumption later). So seeing post C effects the belief that 𝓑=*there is video evidence that bigfoot exists* which effects the belief 𝓐=*bigfoot exists* which effects votes 𝐴.

Our causal graph looks like this.

        sC
         ↓
    sB → 𝓑 → 𝐵
         ↓
         𝓐 → 𝐴

Now, what can we do with this causal graph? Well, to know how a user would change their upvotes 𝐴, after being exposed to all available information (namely posts C and B) we would need to ask them. But assuming the a user's beliefs are modeled by the graph above, it is sufficient instead to know the relationship between 𝐵 and 𝐴, and the relationship between sC and 𝐵. We don't even need any single user to express any opinions about all three posts.

TODO: work out the math






