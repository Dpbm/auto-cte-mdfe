import { useState } from "react"

import type { ButtonClick } from '@customTypes/events';
import type { RateioData } from '@customTypes/api_data';

import { getData } from "./utils/api";

import { Button } from "./components/button";
import { TextArea } from "./components/textarea";
import { DataShow } from "./components/data"


function App() {
  const [query, setQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<RateioData|null>(null);
  const [error,setError] = useState<string|null>(null);

  async function handleSend(event:ButtonClick){
    if(!event || !query) return;
    event.preventDefault();
    event.stopPropagation();

    setLoading(true);
    const {data,error} = await getData(query);
    setData(data);
    setError(error);
    setLoading(false);
  }

  return (
    <main className="flex justify-between p-5 h-screen">
      <aside className="w-100 h-full p-2">
        <TextArea callback={(event) => setQuery(event.target.value)} value={query} />
        <Button onClick={(event) => handleSend(event)} />
      </aside>
      <div className="w-full flex flex-col justify-start">
        {loading ? <p>Loading....</p> : <DataShow data={data} error={error}/>}
      </div>
    </main>
  )
}

export default App;
