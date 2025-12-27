import type { RateioData } from '@customTypes/api_data';

type APIReturn = {
  data: RateioData | null; 
  error: string | null;
};

export async function getData(data:string) : Promise<APIReturn> {
  const url = `${import.meta.env.VITE_API_BASE_URL}/data`
  try{
    const result = await fetch(url, {
      method: "POST",
      headers: {
        'Content-type': 'text/plain',
      },
      body: data
    });
	
    const returnData = await result.json();

    return {
      data: returnData,
      error: null
    };
  }catch(error){
    return {
      data: null,
      error: String(error)
    };
  }
}
