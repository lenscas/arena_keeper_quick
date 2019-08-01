export type file = {
	isDir : boolean,
	name : string,
	path : string
}

export type fileContainer = {
	type : "file",
	file : file
} | {
	type : "dir",
	files : Array<fileContainer>
}
export type assetEnum = {
    name : string,
    path : string
    type : "image" | "font"
}