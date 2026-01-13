import type { LoadData, RateioData, Delivery } from '@customTypes/api_data';

import { copyToClipboard } from "../utils/clipboard"
import { useState } from 'react';

type DataShowProps = {
  data:RateioData|null,
  error:string|null
};

type DataTableProps = {
	deliveries: Delivery[],
};

function fixPrice(price:number):string{
	return price.toString().replaceAll(".",",");
}


const DataTable = ({
	deliveries,
}:DataTableProps) => {

	const [highlight, setHighlight] = useState(0);

	return <table className="table-fixed border-separate border-spacing-y-2">

		<colgroup>
			<col className="w-1/10"/>
			<col className="w-3/10"/>
			<col className="w-1/10"/>
			<col className="w-1/10"/>
			<col className="w-1/10"/>
			<col className="w-1/10"/>
			<col className="w-2/10"/>
		</colgroup>

		  <thead>
		    <tr>
		      <th className="border-b"></th>
		      <th className="text-start border-b">Para</th>
		      <th className="text-start border-b">Frete</th>
		      <th className="text-start border-b">quantidade</th>
		      <th className="text-start border-b">cubicagem</th>
		      <th className="text-start border-b">danfes</th>
		      <th className="text-start border-b">chaves</th>
		    </tr>
		  </thead>
		  <tbody>
		    {
		      deliveries.map((item,index) => 
			  <tr key={index} 
			  	  onClick={() => setHighlight(index)} 
				  className={highlight == index ? "bg-blue-500 text-white" : ""}>
			<td className="text-center border-b py-4 px-2">{index+1}</td>
			<td className="border-b py-4 px-2 brek-all">{item.to}</td>
			<td 
				className="cursor-copy border-b py-4 px-2"
				onClick={() => copyToClipboard(fixPrice(item.price))}>
				R$ {fixPrice(item.price)}
			</td>
			<td 
				className="cursor-copy border-b py-4 px-2"
				onClick={() => copyToClipboard(item.quantity.toString())}>
				{item.quantity}
			</td>
			<td className="border-b py-4 px-2">{item.cubicage}</td>
			<td className="border-b py-4 px-2">{item.danfe.map((danfe,index) => 
					<p className="cursor-copy break-all" 
						key={index} 
						onClick={() => copyToClipboard(danfe)}>{danfe}</p>)
				}
			</td>
			<td className="border-b py-4 px-2">{item.key.map((key,index) => 
					<p className="cursor-copy break-all" 
						key={index} 
						onClick={() => copyToClipboard(key)}>{key}</p>)}
			</td>
		   </tr>)} 

		  </tbody>
	</table>
};

export const DataShow = ({data,error}:DataShowProps) => {
  if(error) return <p className="text-4xl text-red-600 p-10">Erro: {error}</p>;

  const loads : LoadData | null = !data ? null : data.loads;
  if(!loads) return <p className="text-4xl p-10">Nenhum valor a mostrar</p>;

  const carriers = Object.keys(loads);
  if(carriers.length <= 0) return <p className="text-4xl p-10">Nenhuma transportadora!</p>;

  const errors : string[] = !data ? [] : data.errors;

  return <div className="p-5 h-full">
		<ul className="max-h-3/4 min-h-3/4 overflow-scroll">{
		  carriers.map((carrier) => {
			const {loads:LoadsData, sequence, email} = loads[carrier];

			return <li key={carrier}>
				<h1 className="text-3xl sticky top-0 bg-white mb-5">Cargas - {carrier}</h1>
				<pre className="text-xs italic cursor-copy mb-5" onClick={() => copyToClipboard(email)}>{email}</pre>

				{(sequence.length <= 0) ? 
					<p className="text-red-700 mb-10">Nenhum dado para a transportadora!</p> :

					sequence.map((loadNumber) => {
						const loadData = LoadsData[loadNumber];
						const deliveries = loadData.deliveries;

						return <div key={loadNumber} className="p-5 mb-10 ">
							<header className="mb-5">
								<h1 className="text-xl bold">Carga {String(loadNumber)}</h1>
								<h2 className="text-xs italic">{loadData.license_plate.toUpperCase()} - R$ {fixPrice(loadData.total_price)} - {loadData.total_cubicage}</h2>
							</header>
							<DataTable deliveries={deliveries}/>

						</div>;
					})}
				</li>;
			})
		  
		  }</ul>

	  <footer className="max-h-1/4 overflow-scroll">
		  <h1 className="text-2xl text-red-500 sticky top-0 bg-white">Erros durante o rateio</h1>
		  {errors.length <= 0 ? 
			<p>Nenhum erro encontrado!</p> :
			  <ul>
				  {errors.map((error,i) => <li key={i}>{error}</li>)}
			  </ul>
		}
	  </footer>
	</div>;
}
