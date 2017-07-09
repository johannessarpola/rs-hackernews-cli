# CLI For HackerNews

App is aimed to be a cli to 'browse' HackerNews from the comfort of terminal. This is a personal 'hackathon' project I started for fun and to learn some of the interesting things in Rust.

Somewhat working features:
- Next and back viewing top stories, 10 at a time
- Downloading the linked page to local folder <sub>(currently just goes to root of the application)</sub>
- Open the link with your default browser (should work with most OS's with the help of [webbrowser](https://crates.io/crates/webbrowser))
- View comments for story and replies to comments

Commands to use:
- top = opens the currently opened page of stories (reprints)
- next = retrieves the next 10 stories
- back = retrieves the previous 10 stories
- comments [num] = retrieves comments for given story, based on the id of the story shown in [num]
- expand [num] = once comments are open you can retrieve the sub comments for the comment with it
- load [num] = loads the page linked in the story as local html
- open [num] = opens the link with default browser
- exit = quits the application

Operations should be mostly working but there are some bugs still left in the code. Some are written down in errors.txt and some ideas for upcoming features are in features.txt.

Contributions and feedback are welcome!

I've kept a manual bug tracking at errors.txt, but you can create tickets as well.
