import { StepProps, PointType } from '../util/STATIC_DB';

export function Step(props: StepProps) {
  const { id, imageURI, seconds, title, points } = props;
  const imageURIRoot = ''
  // TODO: render time into human readable

  return (
    <div className="rounded-lg shadow flex border sm:flex-row flex-col-reverse bg-white mb-8 sm:h-80">
      <picture>


        <img
          // Or else this if avif not supported:
          // src={imageURIRoot + imageURI}
          src="https://github.com/identicons/jasonlong.png"
          alt=""
          className="rounded-b-lg sm:rounded-l-lg sm:rounded-r-none w-full sm:w-80" />
      </picture>

      <div className="flex flex-col pt-4 pl-4 sm:pt-3 w-full">

        <div>
          <div className="flex flex-row mr-4">
            <h3 className="text-xl font-medium mr-auto pr-1 leading-tight">
              {title}
            </h3>
            <div className="whitespace-nowrap text-sm mt-0.5 mr-0.5 ml-1">
              ⏱ <strong>{seconds}</strong>s
            </div>
          </div>
        </div>
        <ul className="list-outside list-disc pl-5 overflow-auto mt-2 mb-3 pr-2">
          {points
            .sort((a, b) => a.pointType - b.pointType)
            .map((point) => {
              const { id, pointType, title } = point;
              if (pointType === PointType.Warn) {
                return (
                  <li key={id} className="text-red-600">
                    {title}
                  </li>
                );
              } else {
                return <li key={id}>{title}</li>;
              }
            })}
        </ul>
      </div>
    </div>
  );
}
