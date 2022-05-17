
# leafedit
**leafedit** 
is a command line pdf editor written entirely in rust, that can be used to automate repetative pdf manipulation tasks.

## Usage:
### Patch:
before any operation can be applied the pdf file must be patched, usage:

`leafedit patch <INPUT> <OUTPUT>`

example:

`leafedit patch myfile.pdf patched.pdf` will patch myfile.pdf and save it as patched.pdf

### Edit:
operations can now be applied, \
usage:\
`leafedit edit -o 'operation' <INPUT> <OUTPUT>`

mulitple operations can be applied at once:\
`leafedit edit -o 'operation' -o 'operation' -o 'operation' <INPUT> <OUTPUT>`

alternatively a path to a file conating a single operation on each line can be supplied as such:\
`leafedit edit -f <PATH_TO_OPERATIONS_FILE> <INPUT> <OUTPUT>`

operation example: "Wr", which adds a string to a pdf, usage:\
`leafedit edit -o 'Wr (<int>, <int>, <int>, "<string>")' <INPUT> <OUTPUT>`
```
first argument x-coordinate in the pdf content graph

second argument y-coordinate in the pdf content graph

third argument font size

fourth argument string to add to pdf
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
leafedit edit  \
    -o 'Wr (100, 620, 16, "Ahmed Alaa Gomaa")'  \
    -o 'Wr (435, 620, 16, "20201701804")'  \
    -o 'Wr (178, 374, 12, "17.8/20")'  \
    -o 'Wr (513, 374, 14, "89")'  \
    -o 'Wr (513, 340, 14, "70")'  \
    -o 'Wr (132, 406, 18, "✓")'  \
    -o 'Wr (411, 266, 18, "\u{2713}")'  \
    temp.pdf final.pdf
```

and here is the output

![pdf after Wr](images/uel_patched_and_edited.png)

to get a list of all operations run: `leafedit list operations`\
or read the file src/list/\_Operations
