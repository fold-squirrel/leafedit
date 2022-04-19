### Creating Uel paper is a tedious task waiting to be automated

#**ReadMe Deprecated**
# leafedit
**leafedit** 
is a general purpose command line pdf editor (well not just yet but will very soon) written entirely in safe rust, that can be used to automate repetative pdf manipulation tasks.

## Usage:
### Patch:
before any operation can be applied the pdf file must be patched, usage:

`leafedit patch <in_file> [out_file = "patched.pdf"]`

example:

`leafedit patch myfile.pdf` will patch myfile.pdf and save it as patched.pdf

### Edit:
operations can now be applied safely, usage:

`leafedit edit -o 'operation' [in_file = "patched.pdf"] [out_file = "out.pdf"]`

mulitple operations can be applied at once:

`leafedit edit -o 'operation' -o 'operation' -o 'operation' [in_file = "patched.pdf"] [out_file = "out.pdf"]`

currently only one operation is implemted,

operation: "addstr", which adds a string to a pdf, usage:

`leafedit edit -o 'addstr (x:<int>, y:<int>, f:<int>, t:"<string>")' [in_file = "patched.pdf"] [out_file = "out.pdf"]`
```
"x" x-coordinate in the pdf content graph

"y" y-coordinate in the pdf content graph

"f" font size

"t" string to add to pdf, 
to insert emoji in a string like this "yes✓", "yes/Correct/",
anything between '/' will be treated as emoji,
mulitple consecutive emojis can be written 
like this "more than one emoji /Correct,Joy,Coding/",
and finally to have '/' in a string like in "17/20" 
will look like t:"17//20",
```
#### example:

![pdf before addstr](images/uel_patched.png)

first we must patch the file

`leafedit patch uel.pdf temp.pdf`

even though the file appears unchanged
some every important operations where preformed on the pdf
to make edits consistent and speed up editing operations
as editing a patched pdf is much faster that having to patch then edit every time

   

now we apply the edits
```
leafedit edit \
	-o 'addstr (x:100, y:620, f:16, t:"Ahmed Alaa Gomaa")'  \
	-o 'addstr (x:435, y:620, f:16, t:"20201701804")'  \
	-o 'addstr (x:178, y:374, f:12, t:"17.8//20")'  \
	-o 'addstr (x:513, y:374, f:14, t:"89")'  \
	-o 'addstr (x:513, y:340, f:14, t:"70")'  \
	-o 'addstr (x:132, y:406, f:18, t:"/Correct/")'  \
	-o 'addstr (x:411, y:266, f:18, t:"/Correct/")'  \
	temp.pdf final.pdf
```

and here is the output

![pdf after addstr](images/uel_patched_and_edited.png)

more coming  soon.
