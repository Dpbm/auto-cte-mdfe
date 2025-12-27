import type { LoadData, RateioData, Delivery } from '@customTypes/api_data';

import { copyToClipboard } from "../utils/clipboard"

type DataShowProps = {
  data:RateioData|null,
  error:string|null
};

type DataTableProps = {
	deliveries: Delivery[]
};


const DataTable = ({deliveries}:DataTableProps) => (
	<table className="table-fixed border-separate border-spacing-y-2">

		<colgroup>
			<col class="w-2/8"/>
			<col class="w-1/8"/>
			<col class="w-1/8"/>
			<col class="w-1/8"/>
			<col class="w-1/8"/>
			<col class="w-2/8"/>
		</colgroup>

		  <thead>
		    <tr>
		      <th className="text-start">Para</th>
		      <th className="text-start">Frete</th>
		      <th className="text-start">quantidade</th>
		      <th className="text-start">cubicagem</th>
		      <th className="text-start">danfes</th>
		      <th className="text-start">chaves</th>
		    </tr>
		  </thead>
		  <tbody>
		    {
		      deliveries.map((item) => <tr key={item.to}>
			<td>{item.to}</td>
			<td 
				className="cursor-copy"
				onClick={() => copyToClipboard(item.price.toString())}>
				R$ {item.price}
			</td>
			<td 
				className="cursor-copy"
				onClick={() => copyToClipboard(item.quantity.toString())}>
				{item.quantity}
			</td>
			<td>{item.cubicage}</td>
			<td>{item.danfe.join("\n")}</td>
			<td>{item.key.join("\n")}</td>
		      </tr>)
		    } 

		  </tbody>
	</table>
);

export const DataShow = ({data,error}:DataShowProps) => {
  if(error) return <p className="text-4xl text-red-600 p-10">Erro: {error}</p>;

  const loads : LoadData | null = !data ? null : data.loads;
  if(!loads) return <p className="text-4xl p-10">Nenhum valor a mostrar</p>;

  const carriers = Object.keys(loads);
  if(carriers.length <= 0) return <p className="text-4xl p-10">Nenhuma transportadora!</p>;

  const errors = data.errors;

  return <div className="p-5 h-full">
		<ul className="max-h-3/4 min-h-3/4 overflow-scroll">{
		  carriers.map((carrier) => {
			const carrierData = loads[carrier];
			const loadNumbers = Object.keys(carrierData);

			return <li key={carrier}>
				<h1 className="text-3xl sticky top-0 bg-white mb-5">Cargas - {carrier}</h1>

				{(loadNumbers.length <= 0) ? 
					<p className="text-red-700 mb-10">Nenhum dado para a transportadora!</p> :


					loadNumbers.map((loadNumber) => {
						const loadData = carrierData[loadNumber];
						const deliveries = loadData.deliveries;

						return <div key={loadNumber} className="p-5 mb-10 border-1 border-stone-800 rounded-xl shadow-md">
							<header className="mb-5">
								<h1 className="text-xl bold">Carga {String(loadNumber)}</h1>
								<h2 className="text-xs italic">{loadData.license_plate.toUpperCase()} - R$ {loadData.total_price} - {loadData.total_cubicage}</h2>
							</header>
							<DataTable deliveries={deliveries} />

						</div>;
					})}
				</li>;
			})
		  
		  }</ul>

	  <footer className="max-h-1/4 overflow-scroll">
		  <h1 className="text-2xl text-red-500 sticky top-0 bg-white">Erros durante o rateio</h1>
		  <ul>
			  {errors.map((error,i) => <li key={i}>{error}</li>)}
		  </ul>
	  </footer>
	</div>;
}
