# reproducible-builds-certification-repository

Collecting some thoughts and prototyping for a [certification repository](https://reproducible-builds.org/docs/sharing-certifications/)
for [reproducible builds](https://reproducible-builds.org)

Debian has a proof-of-concept [buildinfo server](https://github.com/lamby/buildinfo.debian.net),
might be interesting to consider if that is or can be made suitable for more
general use.

## Storage

Various 'sophisticated' storage mechanisms were considered below, but for
now none seem to fit elegantly. For now let's just put the files on a
regular filesystem and allow mirroring with 'traditional' tools like rsync.

### Append-only logs

It has been proposed to use a secure append-only log similar to the one used
in [Certificate Transparency](https://www.certificate-transparency.org). The
advantage of such a log is that a rogue certification server couldn't filter
out some certifications and include their own instead.

However, we want it to be low-friction even for 'non-trusted' participants to
add certifications to the repository. This puts us at risk of people polluting
the repository with meaningless certifications, which means it would be useful to
be able to do maintenance and clean it up - which seems directly at odds with using 
a secure append-only log.

### Distributed storage

It would be good if a certification repository doesn't have to rely on a single
server (or credit card) to store data. Some kind of distributed storage would be
interesting.

#### IPFS

IPFS is content-adressible, meaning content can be found by learning its hash
and looking for it. Since the query pattern for reproductions is probably
mostly 'what are the certifications for package X', we would need to use
something like ipns to make the mapping, but that means writing to the repo
can only be done by someone who owns a particular private key, which would be
nice not to need.

#### Others?

Sounds like we'd be looking for a kind of 'open dropbox alternative',
preferably one where it's easy to set up a read-only node and configure
replication/peering afterwards.

* ownCloud/Nextcloud are PHP which does not feel right
* seaFile (mostly C, also not great) might work but has GC issues.

## Format

Debian has standardized on using OpenPGP-signed [.buildinfo](https://anonscm.debian.org/cgit/reproducible/buildinfo-spec.git/tree/notes/buildinfo.rst) files, which are
formatted as Debian 'control files'. This might seem a bit odd outside of Debian,
but it might be a good start to follow that convention at least for now,
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

Both for reading and writing, we need some way to identify exactly what binary is
being certified. It probably makes sense to have separate certification repository
deployments for separate collections of binaries: for example, a certification
repository for Debian packages would use the Debian package name, version and
architecture. A repository for the jars at [Maven Central](https://search.maven.org)
would likely include groupId, artifactId, version, classifier and resource type.

## Implementation

When choosing a simple HTTP interface for reading and writing, and storing the
certifications on disk, the implementation is pretty close to a traditional
webserver. There are only some small things missing: deciding what POSTs to allow
and serving machine-readable folder indexes. This could be implemented by using
a webserver as a starting point and writing some custom extensions, or by writing
some software to perform exactly this and act as a webserver. With the right
libraries the latter would likely be easiest, and for the more advanced
webserver features you could put a webserver acting as a proxy in between.

I'd like this software to be low-footprint and nicely typed, possibly rust
would be a nice choice?
