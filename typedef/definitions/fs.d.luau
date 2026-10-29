
--[=[
	@class File

    An Object representing a file.
]=]
declare extern type File with
    read Path: string
    read Size: number
    read Created: DateTime
    read Accessed: DateTime
    read Modified: DateTime
    read Exists: boolean
    read Extension: boolean
    --[=[
        @within File
        @method Read
        @returns Contents of the file.

        Reads the contents of a file as returns it as a buffer.
    ]=]
    function Read(self): buffer
    --[=[
        @within File
        @method Write
        @param contents The contents of the file.
        @return Itself

        Writes the given contents into a file.
    ]=]
    function Write(self,contents : buffer) : File
    --[=[
        @within File
        @method Append
        @param contents The contents of the file.
        @return Itself

        Appends the given contents onto the end of a file.
        If the file does not exist it will instead create a new file to append to.
    ]=]
    function Append(self,contents : buffer) : File
    --[=[
        @within File
        @method Delete

        Deletes the file.
    ]=]
    function Delete(self)
    --[=[
        @within File
        @method Move
        @param path The path to move to.
        @return Itself
        
        Moves the file to a different location.
    ]=]
    function Move(self,path: string) : File
    --[=[
        @within File
        @method Copy
        @param newPath The path to copy to.
        @return The copy
        
        Copies the file to a new location.
    ]=]
    function Copy(self,path: string) : File

    function __tostring(self): string
end

--[=[
	@class Directory

    An Object representing a file directory.
]=]
declare extern type Directory with 
    read Path: string
    read Created: DateTime
    read Accessed: DateTime
    read Modified: DateTime
    read Exists: boolean
    --[=[
        @within Directory
        @method Make
        @return Itself
        
        Creates a file directory where the directory object's path is located.
    ]=]
    function Make(self) : File
    --[=[
        @within Directory
        @method Move
        @param path The path to move to.
        @return Itself
        
        Moves the directory to a different location.
    ]=]
    function Move(self,path: string) : File
    --[=[
        @within Directory
        @method Delete

        Deletes the file directory and all files inside it.
    ]=]
    function Delete(self)
    --[=[
        @within Directory
        @method Walk
        @return iterator

        Returns an non recursive iterator that walks through every file and directory in that directory.
    ]=]
    function Walk(self) : () -> File | Directory

    function __tostring(self): string
end

--[=[
	@class FsObserver

    A object that watches files and directories.
]=]
declare extern type FsObserver with 
    read Path: string
    OnCreated : (paths: {string}) -> ()
    OnModified : (paths: {string}) -> ()

    --[=[
        @within FsObserver
        @method Disconnect

        Stops the observer from observing file events.    
    ]=]
    function Disconnect(self)

end