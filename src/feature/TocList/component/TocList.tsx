type Props = {
  toc: Toc;
}

export function TocList(props: Props) {
  return <section>
    <h2>TOC</h2>
    <ol>
      {props.toc.trackData.map(track => <li>{track.minutes.toString().padStart(2, ' ')}:{track.seconds.toString().padStart(2, '0')}</li>)}
    </ol>
  </section>
}