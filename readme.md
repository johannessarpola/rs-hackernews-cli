# CLI For HackerNews

App is aimed to be a cli to 'browse' HackerNews from the comfort of terminal. This is a personal 'hackathon' project I started for fun and to learn some of the interesting things in Rust. 

Currently source is quite unorganized and littered with FIXMEs and TODOs

Commands to use:
- top = opens the currently opened page of stories (reprints)
- next = retrieves the next 10 stories
- back = retrieves the previous 10 stories
- comments num = retrieves comments for given story, based on the id of the story shown in [num]
- expand num = once comments are open you can retrieve the sub comments for the comment with it 
- exit = quits the application
- load num = loads the page linked in the story as local html

Somewhat working features:
- Next and back viewing top stories, 10 at a time
- Downloading the linked page to local folder (currently just goes to root of the application)
- Open the link with your default browser (should work with most OS's with the help of Webrowser crate)
- View comments for story and subcomments

**This will be on hiatus for an undefined amount of time after 0.0.1 as there's some other things I need to do.** Not sure on when I'll return to this.

Contributions and feedback are welcome!
