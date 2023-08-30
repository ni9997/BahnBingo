import { Reason } from './Reason';

export function getReasons() {
	return [
		new Reason('Signalstörung'),
		new Reason('Reperatur am Zug'),
		new Reason('Stellwerksstörung'),
		new Reason('Verspätete Bereitstellung des Zuges'),
		new Reason('Verspätung des Fahrpersonals'),
		new Reason('Reperatur an einer Weiche'),
		new Reason('Reperatur an einer Oberleitung'),
		new Reason('Warten auf Anschlussreisende'),
		new Reason('Notarzteinsatz am Gleis'),
		new Reason('Gleis belegt'),
		new Reason('Verspätung aus vorheriger Fahrt'),
		new Reason('Planmäßige Verspätung'),
		new Reason('Bombenentschärfung'),
		new Reason('Polizeieinsatz'),
		new Reason('Kinder auf den Gleisen'),
		new Reason('Wetter'),
		new Reason('Streik'),
		new Reason('Defekter Zug'),
		new Reason('Kurzfristiger Personalausfall'),
		new Reason('Zug wird von einem ICE überholt'),
		new Reason('Gleislagefehler'),
		new Reason('Unregelmäßigkeiten Bau'),
		new Reason('Unfall mit Personenschaden'),
		new Reason('Streckensperrung'),
		new Reason('Umleitung'),
		new Reason('Zug hängt hinter einer S-Bahn fest'),
		new Reason('Böschungsbrand'),
		new Reason('Defekt an einem Bahnübergang'),
		new Reason('Zug hat sich verfahren'),
		new Reason('Verspätete Bereitstellung des Zuges'),
		new Reason('Haltestelle verpasst')
	];
}
