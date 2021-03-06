extern crate natural;

use natural::classifier::NaiveBayesClassifier;

#[test]
fn test_basic_usage() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str("Debug derive traits into impls that I use with my rust code"), String::from_str("rust"));
  nbc.train(String::from_str("deriving show for your impl can definitely help you debug your rust code"), String::from_str("rust"));
  nbc.train(String::from_str("Use more metaprogramming when writing ruby"), String::from_str("ruby"));
  nbc.train(String::from_str("Classes can have either instance variables or class variables"), String::from_str("ruby"));
  assert_eq!(nbc.guess(String::from_str("This class is about ruby")), String::from_str("ruby"));
  assert_eq!(nbc.guess(String::from_str("debug this rust code")), String::from_str("rust"));
}

#[test]
fn test_medium_dataset() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str("Why do I want to write the 234th comment on The Shawshank Redemption? I am not sure - almost everything that could be possibly said about it has been said. But like so many other people who wrote comments, I was and am profoundly moved by this simple and eloquent depiction of hope and friendship and redemption. \
The only other movie I have ever seen that effects me as strongly is To Kill a Mockingbird. Both movies leave me feeling cleaner for having watched them. \
I didn't intend to see this movie at all: I do not like prison movies and I don't normally watch them. I work at a branch library and one day as I was checking The Shawshank Redemption out to one of our older patrons, she said to me, Whenever I feel down or depressed, I check out this movie and watch it and it always makes me feel better. At the time, I thought that was very strange. One day there was nothing on TV except things I absolutely would not watch under any circumstance or things that I had seen too many times already. I remembered what she said, so I watched it. I have watched it many many times since then and it gets better with every showing. \
No action, no special effects - just men in prison uniforms talking to each other. \
The Shawshank Redemption and To Kill a Mockingbird are the best movies I have ever seen. I do not judge it by it's technical merits - I don't really care about that. I have read that Citizen Kane or The Godfather or this or that movie is the best movie ever made. They may have the best technique or be the most influential motion pictures ever made, but not the best. The best movies are ones that touch the soul. It takes a movie like The Shawshank Redemption to touch the soul."), String::from_str("positive"));


  nbc.train(String::from_str("Can Hollywood, usually creating things for entertainment purposes only, create art? To create something of this nature, a director must approach it in a most meticulous manner, due to the delicacy of the process. Such a daunting task requires an extremely capable artist with an undeniable managerial capacity and an acutely developed awareness of each element of art in their films, the most prominent; music, visuals, script, and acting. These elements, each equally important, must succeed independently, yet still form a harmonious union, because this mixture determines the fate of the artist's opus. Though already well known amongst his colleagues for his notable skills at writing and directing, Frank Darabont emerges with his feature film directorial debut, The Shawshank Redemption. Proving himself already a master of the craft, Darabont managed to create one of the most recognizable independent releases in the history of Hollywood. The Shawshank Redemption defines a genre, defies the odds, compels the emotions, and brings an era of artistically influential films back to Hollywood.\
The story begins with the trial of a young banker, Andy Dufrense, victimized by circumstantial evidence, resulting in a conviction for the murder of his wife and her lover. After a quick conviction, Andy finds himself serving a life sentence at Shawshank prison, with no hope of parole. He exists in this prison only in appearance, keeping his mind free from the drab walls around him. His ability to do this results in the gaining of respect from his fellow inmates, but most of all from Ellis Redding. Ellis, commonly referred to as Red, finds gainful use of his entrepreneurial spirit within the drab walls of Shawshank by dealing in contraband and commodities rare to the confines of prison. Andy's demeanor and undeniable sense of hope causes Red to take a deeper look at himself, and the world around him. Andy proves to Red and the other inmates that in the conventional walls of Shawshank prison convention will find no home in his lifestyle.\
By creating the film's firm foundation, the meticulously chiseled screenplay paved the way for this film's success. Frank Darabont outdoes himself with the phenomenal adaptation of Stephen King's equally noteworthy novella, Rita Hayworth and Shawshank Redemption. In this novella, King demonstrates that he can break free from the genre he dominates and still create a marvelous piece of modern literature. Though the film mirrors the novella in many ways, Darabont illustrates a focused objective of improving upon the areas where the novella came up short, resulting in one of the best book to film transitions ever.\
While maintaining some of the poetic and moving dialogue of the novella, Darabont also proves that a film's score can generate a great deal of emotional response from its audience, as dialogue does. He employs the cunning Thomas Newman, son of the legendary Hollywood composer, Alfred Newman. Darabont shows recognition for the film's needs by employing Newman, who makes the gentle piano chords whisper softly to the viewer, as if a part of the scripted dialogue. Newman lends himself to individualism and tends to drive more towards the unique in the realm of score composition. His effort in Shawshank did not go unnoticed, as his score received an Oscar nomination in 1995. While unique and independent, Newman's score never once intrudes on your concentration or distracts from the film.\
With work from vast array of talented scene designers, costume designers, composers, cinematographers, and various other Hollywood artists, the cast of The Shawshank Redemption had a strong foundation to work with. The marvelous cast of this film will dazzle you with some of the most convincing performances you will witness in a film. While both Tim Robbins and Morgan Freeman shine as Andy and Red, respectively, the true spectacle of acting lies within the plethora of amazing supporting actors who easily disappear into their roles. Most noticeable of these, the veteran film star James Whitmore, who portrays the elderly Brooks Hatlen. Brooks, a man incarcerated for an unmentioned crime for so long that he finds himself attached to the Shawshank and the daily life he has lead. Each of these actors show a true dedication to their art, and a focused purpose in their motivations, creating a convincing setting that never once caters to anything unbelievable.\
With all of the aesthetic touches and attention to cinematic detail, the most beautiful part of the film lies within its thematic material, such as its focus on the human desires for the most abstract concepts, like hope and freedom. These themes, which concern things the human spirit undoubtedly yearns for, seem so intricately woven into the plot that it easily draws its audience in to its story. Though full of hardened criminals, your heart will go out to these men as they display the most basic of human emotions, and deliver some of the most quotable lines in a film to date. Like a great novel, this film manages to succeed at greater things than simply entertaining an audience. Darabont tells his story most masterfully, illustrating principles and inspiring his audience to think. He leaves us a poignant film with a powerful message of hope, and redemption, something we all seek.\
This film manages to redeem Hollywood in the eyes of people who feared it long lost in a dark sea of cliches and predictability. Darabont shows us that artists still work in the Hollywood studios and production facilities. These artists show their capability to produce art; real art that inspires you to look at the deeper aspects of life and the world around you. The Shawshank Redemption delivers much-needed breath of fresh air for anyone who realizes the capability of film. It proves that masters of the craft still live on this earth, and still bless us with timeless masterpieces that we will never forget."), String::from_str("positive"));

  nbc.train(String::from_str("I believe that this film is the best story ever told on film, and I'm about to tell you why.\
Tim Robbins plays Andy Dufresne, a city banker, wrongfully convicted of murdering his wife and her lover. He is sent to Shawshank Prison in 1947 and receives a double life sentence for the crime. Andy forms an unlikely friendship with Red (Morgan Freeman), the man who knows how to get things. Andy faces many trials in prison, but forms an alliance with the wardens because he is able to use his banking experience to help the corrupt officials amass personal fortunes. The story unfolds....\
I was so impressed with how every single subplot was given a great deal of respect and attention from the director. The acting was world-class. I have never seen Tim Robbins act as well since, Morgan Freeman maybe (e.g. Seven). The twists were unexpected, an although this film had a familiar feel, it wasn't even slightly pretentious or cliched, it was original. The cinematography was grand and expressive. It gave a real impression of the sheer magnitude of this daunting prison.\
But the one thing which makes THE SHAWSHANK REDEMPTION stand above all other films, is the attention given to the story. The film depends on the story and the way in which it unravels. It's a powerful, poignant, thought-provoking, challenging film like no other. If Andy were to comment on this film, I think he might say: Get busy watching, or get busy dying. Take his advice.\
Thoroughly recommended."), String::from_str("positive"));

  nbc.train(String::from_str("At the time of this review, 'The Shawshank Redemption' is the number one movie on IMDb. Why? If anything, it is probably the most cliched film I have ever seen with my own two eyes, and yet it is rated as the top film of all time? It blows my mind every time I visit this website.\
This 'film' was adapted from a novel by the renowned author Steven King. I have not read the book, as I have already been spoiled by the movie, but apparently the writing is quite exquisite. However, Frank Darabont must have missed the entire point of Steven's works, which is to have engaging characters that you are able to sympathize with.\
The characters are so bland, I might as well be staring at glue dry. Tim Robbins and Morgan Freeman's bromance literally develops right out of the blue after they have one conversation. Pacing? What's that? Later, you meet an old fart named Brooks, who might as well have his name changed to I'm going to die later so feel sad-larry, and the most he contributes to the film is increasing the runtime with his sub-plot about being outside after being locked in for so long. One cannot forget the oh-so-memorable (sarcasm) warden who channels the spirits of every Disney villain without the accompanying song. You can find more interesting characters in a Tyler Perry movie.\
The worst part of the movie is definitely the passage of time. The story takes place over twenty years or so and yet the only one to actually age is the warden. Tim and Morgan stay in some special cells where they magically do not get older and therefore nobody notices on IMDb. While a competent director would have noticed this as soon as they glanced at the script, Darabont was off in the broom closet shagging the catering chick or something.\
If you enjoy modern shooters then you are in for a treat. Most of this movie is full of grays, browns, mixes between, and it even has a sewer section just for the hell of it. Just when you thought the movie might get interesting when out of the prison, prepare to be disappointed. Honestly, the best shot of the movie is when Tim is crawling through sh*t, as it reflects on my feelings towards the film.\
Music can make - or break - a movie. Here, the 'original' score only seems to try and make you cry at the most inappropriate times. The music is too overplayed in films like this, where it takes away from the already terrible experience.\
This movie is as lame as they come. If you like this, fine. As the phrase says, ignorance is bliss."), String::from_str("negative"));

  nbc.train(String::from_str("The Shawshank Redemption is a movie about a hotshot banker named Andy Dufresne(Tim Robbins) who is convicted of murdering his wife and is imprisoned in Shawshank Prison. He remains hopeful and makes friends especially with a blackmarketeer named Red ( Morgan Freeman). I really don't see how this movie could possibly be the #2 film of all time, the movie is nothing groundbreakingly amazing, it is pretty poorly directed, the plot is thinner than air,there are no risks taken,and there is no thought provoked through watching the film that it only needs to one viewing to get the whole movie, in fact you wouldn't even need to watch the film to even understand how the movie is going end. The title already gives away the whole big finale of the movie. I don't see how anyone could possibly compare THIS to actually great movies like EVERYTHING that is below is on the Top 25. And the simple thought of this winning Best Picture back in '94, HAHAHAHAHAHAHA!, it would have given me somethingh to laugh at for the rest of my life, I still can't possibly see how the Academy could have even nominated this as the Best Picture of 1994. Anyway the only thing that saves the film of Morgan Freeman and it is only thing that even got me to see this film in the first place but it still can't stop me from giving this a poor grade."), String::from_str("negative"));

  nbc.train(String::from_str("awful. a dreadful disgrace. the film is the most cliched in cinematic history and is worthy of no respect. even worse is how it is so flawless yet it still is of no quality. i do not know how this film can be revered over others such as psycho, apocalypse now, raging bull, easy rider, the godfather, the killing fields, some like it hot and casablanca amongst others. shockingly horrendous"), String::from_str("negative"));
  
  nbc.train(String::from_str("The notion that this movie belongs in the top 100 of anybody's list is simply retarded. This B-movie with an A-movie list is standard, run of the mill, TV-movie-of-the-week fare. How did it get this high? What's wrong with the vast number of IMDB users that they would vote so highly in favor of such a piece of trite? My theory: Tim Robbins is sending each of you a check. That's the only explanation. This movie offers nothing special: nothing different, nothing unique. No great performances (a phoned-in Morgan Freeman performance akin to his Deep Impact presidential performance), no graphic portrayals or stark realism of life behind bars--nothing. This movie achieves what few in history have done: it both sucks AND blows."), String::from_str("negative"));

  let guess = nbc.guess(String::from_str("The Shawshank Redemption is without a doubt one of the most brilliant movies I have ever seen. Similar to The Green Mile in many respects (and better than it in almost all of them), these two movies have shown us that Stephen King is a master not only of horror but also of prose that shakes the soul and moves the heart. The plot is average, but King did great things with it in his novella that are only furthered by the direction, and the acting is so top-rate it's almost scary.\
Tim Robbins plays Andy Dufrane, wrongly imprisoned for 20 years for the murder of his wife. The story focuses on Andy's relationship with Red Redding (Morgan Freeman, in probably his best role) and his attempts to escape from Shawshank. Bob Gunton is positively evil and frightening as Warden Norton, and there are great performances and cameos all around; the most prominent one being Gil Bellows (late as Billy of Ally McBeal) as Tommy, a fellow inmate of Andy's who suffers under the iron will of Norton.\
If you haven't seen this movie, GO AND RENT IT NOW. You will not be disappointed. It is positively the best movie of the '90's, and one of my Top 3 of all time. This movie is a spectacle to move the mind, soul, and heart. 10/10"));

  assert_eq!(guess, String::from_str("positive"));

}

#[test]
fn no_fail_on_empty_strings() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str(""), String::from_str(""));
  assert!(true);
}