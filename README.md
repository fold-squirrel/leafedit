
# leafedit
**leafedit** 
is a general purpose command line pdf editor (well not just yet but will very soon) written entirely in safe rust, that can be used to automate repetative pdf manipulation tasks.

## Usage:
### Patch:
before any operation can be applied the pdf file must be patched, usage:

`leafedit patch <INPUT> <OUTPUT>`

example:

`leafedit patch myfile.pdf patched.pdf` will patch myfile.pdf and save it as patched.pdf

### Edit:
operations can now be applied safely, usage:

`leafedit edit -o 'operation' <INPUT> <OUTPUT>`

mulitple operations can be applied at once:

`leafedit edit -o 'operation' -o 'operation' -o 'operation' <INPUT> <OUTPUT>`

currently only one operation is implemted,

operation: "Wr", which adds a string to a pdf, usage:

`leafedit edit -o 'Wr (x:<int>, y:<int>, f:<int>, t:"<string>")' <INPUT> <OUTPUT>`
```
"x" x-coordinate in the pdf content graph

"y" y-coordinate in the pdf content graph

"f" font size

"t" string to add to pdf
```
#### example:

![pdf before Wr](images/uel_patched.png)

first we must patch the file

`leafedit patch uel.pdf temp.pdf`

even though the file appears unchanged
some every important operations where preformed on the pdf
to make edits consistent and speed up editing operations
as editing a patched pdf is much faster that having to patch then edit every time

   

now we apply the edits
```
leafedit edit -p A4 \
	-o 'Wr (x:100, y:620, f:16, t:"Ahmed Alaa Gomaa")'  \
	-o 'Wr (x:435, y:620, f:16, t:"20201701804")'  \
	-o 'Wr (x:178, y:374, f:12, t:"17.8/20")'  \
	-o 'Wr (x:513, y:374, f:14, t:"89")'  \
	-o 'Wr (x:513, y:340, f:14, t:"70")'  \
	-o 'Wr (x:132, y:406, f:18, t:"✓")'  \
	-o 'Wr (x:411, y:266, f:18, t:"\u{2713}")'  \
	temp.pdf final.pdf
```

and here is the output

![pdf after Wr](images/uel_patched_and_edited.png)

more coming  soon.
