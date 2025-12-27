export async function copyToClipboard(data:string) {
	try{
		await navigator.clipboard.writeText(data);
	}catch(error){
		console.error(`Failed on copy ${data} to clipboard\n${error}`);
	}
}
