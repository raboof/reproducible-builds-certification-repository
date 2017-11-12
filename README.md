# reproducible-builds-certification-repository

Collecting some thoughts and prototyping for a [certification repository](https://reproducible-builds.org/docs/sharing-certifications/)
for [reproducible builds](https://reproducible-builds.org)

## Storage

### Append-only logs

It has been proposed to use a secure append-only log similar to the one used
in [Certificat Transparency](https://www.certificate-transparency.org). The
advantage of such a log is that a rogue certification server couldn't filter
out some certifications and include their own instead.

However, we want it to be low-friction even for 'non-trusted' participants to
for add certifications to the repository. This puts us at risk to people polluting
the repository with meaningless certifications, which means it would be useful to
be able to do maintenance and clean it up - which seems directly at odds with using 
a secure append-only log.

### Distributed storage

It would be good if a certification repository doesn't have to rely on a single
server (or credit card) to store data. Some kind of distributed storage would be
interesting.

Would IPFS work?

## Format

Debian has standardized on using OpenPGP-signed .buildinfo files, which are
formatted as Debian 'control files'. This might seem a bit odd outside of Debian,
but might it might be a good start to follow that convention at least for now,
or leave it up to each deployment to agree on a convention.

## API

To make it as low-friction as possible to contribute and check certifications,
as well as experiment with different back-end storage mechanisms, it would be
useful to define a convenient API.

### Writing

Writing could be as simple as POSTing a signed .buildinfo file to a predictable
URL. 

### Reading

Collecting certifications could be as simple as fetching them from a predictable
URL. To minimize complexity, it would probably make most sense to first fetch a
list of certification URL's based on a source identifier, and then fetch the actual
certification files in subsequent requests.

### Source identifiers

Both for reading and writing we need some way to identify exactly what binary is
being certified. It probably makes sense to have separate certification repository
deployments for separate collections of binaries: for example, a certification
repository for Debian packages would use the Debian package name, version and
architecture. A repository for the jars at [Maven Central](https://search.maven.org)
would likely include groupId, artifactId, version, classifier and resource type.

