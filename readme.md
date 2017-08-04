# CLI For HackerNews

App is aimed to be a cli to 'browse' HackerNews from the comfort of terminal. This is a personal 'hackathon' project I started for fun and to learn some of the interesting things in Rust with crates like Tokio, Hyper and some others.

## Commands to use

- top = opens the currently opened page of stories (reprints as well)
- next = retrieves the next 10 stories or comments
- back = retrieves the previous 10 stories or comments
- comments [num] = retrieves comments for given story, based on the id of the story shown in [num] ten at a time
- expand [num] = once comments are open you can retrieve the sub comments for the comment with it ten at a time
- load [num] = loads the page linked in the story as local html
- open [num] = opens the link with default browser
- exit = quits the application
- help = prints out in-app help and command reference


## Notes

All the basic functionality should be working but there are some bugs. Some are written down in errors.txt and some ideas for upcoming features are in features.txt. Todos.txt is some meta-chores to be done. 

Contributions and feedback are welcome!
