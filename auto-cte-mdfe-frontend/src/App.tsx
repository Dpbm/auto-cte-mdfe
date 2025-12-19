import { useState } from "react"

import type { LoadData, RateioData } from '@customTypes/api_data';
import type { ButtonClick } from '@customTypes/events';

import { Button } from "./components/button";
import { TextArea } from "./components/textarea";

type APIReturn = {
  data: RateioData | null; 
  error: string | null;
};

async function getData(data:string) : Promise<APIReturn> {
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

type DataShowProps = {
  data:RateioData|null,
  error:string|null
};

const DataShow = ({data,error}:DataShowProps) => {
  if(error) return <p>Falha erro: {error}</p>;

  const loads : LoadData | null = !data ? null : data.loads;
  if(!loads) return <p>Nenhum valor a mostrar</p>;
      console.log(loads);

  const loadsNumbers = Object.keys(loads);


  return <>
    {loadsNumbers.map((loadNumber) => {
      const load = loads[parseInt(loadNumber)];
      const totalPrice = load?.total_price ?? 0.0;
      const totalCubicage = load?.total_cubicage ?? 0.0;
      const licensePlate = load?.license_plate ?? '';
      const transp =  load?.data[0].by ?? '';

      return <div key={loadNumber}>
        <h1>carga: {loadNumber}</h1>
        <h2>por: {transp}; Placa: {licensePlate}; frete: {totalPrice}; cubicagem: {totalCubicage}</h2>
        <table>
          <thead>
            <tr>
              <th>danfe</th>
              <th>de</th>
              <th>para</th>
              <th>carga</th>
              <th>cubicagem</th>
              <th>quantidade</th>
              <th>frete</th>
            </tr>
          </thead>
          <tbody>
            {
              load?.data.map((item) => <tr key={item.danfe}>
                <td>{item.danfe}</td>
                <td>{item.by}</td>
                <td>{item.to}</td>
                <td>{item.load_number}</td>
                <td>{item.cubicage}</td>
                <td>{item.quantity}</td>
                <td>{item.price}</td>
              </tr>)
            } 

          </tbody>
        </table>
      </div>;

    }
    
    )} 
  </>;





}


function App() {
  const [query, setQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<RateioData|null>(null);
  const [error,setError] = useState<string|null>(null);

  async function handleSend(event:ButtonClick){
    if(!event) return;
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
      <aside className="w-100 h-full p-5">
        <TextArea callback={(event) => setQuery(event.target.value)} value={query} />
        <Button onClick={(event) => handleSend(event)} />
      </aside>
      <div>
        {loading ? <p>Loading....</p> : <DataShow data={data} error={error}/>}
      </div>
    </main>
  )
}

export default App
